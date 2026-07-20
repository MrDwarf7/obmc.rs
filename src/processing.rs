use std::borrow::Cow;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, FixedOffset, Local};
use nom_exif::{EntryValue, Exif, ExifTag, MediaKind, MediaParser, MediaSource, TrackInfoTag};

use crate::folders::Folder;
use crate::prelude::*;
use crate::{Separators, stamp_converter};

fn write_handle<W: Write>(handle: &mut W, old: &str, new: &str, dry_run: bool) {
    if dry_run {
        let _ = writeln!(handle, "[DRY RUN] | {old:<20} -> {new:<20}");
    } else {
        let _ = writeln!(handle, "{old:<20} -> {new:<20}");
    }
}

pub fn process_folder(folder: &Folder, dry_run: bool) {
    let std_out = std::io::stdout();
    let mut handle = std_out.lock();
    let seps = Separators::default();
    // Reuse parser buffer across files in this folder.
    let mut parser = MediaParser::new();

    for file in &folder.children_files {
        let old_path = &file.path;
        let Some(old_name) = old_path.file_name().and_then(|n| n.to_str()) else {
            eprintln!("[warn] skipping non-utf8 filename: {}", old_path.display());
            continue;
        };

        let new_name = match convert_name(old_path, old_name, &seps, &mut parser) {
            Ok(ConvertOutcome::AlreadyPrefixed) => {
                let _ = writeln!(handle, "[skip] already prefixed: {old_name}");
                continue;
            }
            Ok(ConvertOutcome::Renamed(name)) => name,
            Err(e) => {
                eprintln!("[err] {}: {e:#}", old_path.display());
                continue;
            }
        };

        let new_path = old_path.with_file_name(&new_name);

        if dry_run {
            write_handle(&mut handle, old_name, &new_name, true);
            continue;
        }

        write_handle(&mut handle, old_name, &new_name, false);
        if let Err(e) = std::fs::rename(old_path, &new_path) {
            eprintln!("[err] rename {} -> {}: {e}", old_path.display(), new_path.display());
        }
    }

    for child_folder in &folder.children {
        process_folder(child_folder, dry_run);
    }

    let _ = handle.flush();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertOutcome {
    /// Filename already starts with the computed date prefix; leave it alone.
    AlreadyPrefixed,
    Renamed(String),
}

/// Pure rename decision from a raw stamp string (no I/O).
///
/// `stamp_raw` is the dual-time form produced by [`format_dual`] /
/// [`get_creation_time`], e.g. `"26/05/2022 14:40 02:40 PM"`.
pub fn rename_from_stamp(old_name: &str, stamp_raw: &str, seps: &Separators<'_>) -> Result<ConvertOutcome> {
    let new_date = stamp_converter::flip_date_format(stamp_raw, seps)?;
    let prefix_with_sep = format!("{new_date}{DATE_TIME_SEP}");

    if old_name.starts_with(&prefix_with_sep) {
        return Ok(ConvertOutcome::AlreadyPrefixed);
    }

    Ok(ConvertOutcome::Renamed(format!("{prefix_with_sep}{old_name}")))
}

/// Fetch creation stamp for `old_path`, then decide the new name.
pub fn convert_name(
    old_path: &Path,
    old_name: &str,
    seps: &Separators<'_>,
    parser: &mut MediaParser,
) -> Result<ConvertOutcome> {
    let creation_time = get_creation_time(old_path, parser)?;
    rename_from_stamp(old_name, &creation_time, seps)
}

/// Dual time stamp string: 24h for sort, 12h+AM/PM for reading.
pub fn format_dual(dt: DateTime<Local>) -> String {
    dt.format("%d/%m/%Y %H:%M %I:%M %p").to_string()
}

fn local_offset() -> FixedOffset {
    *Local::now().offset()
}

fn entry_to_local(ev: &EntryValue) -> Option<DateTime<Local>> {
    let edt = ev.as_datetime()?;
    let fixed = edt.or_offset(local_offset());
    Some(fixed.with_timezone(&Local))
}

fn system_time_to_local(st: SystemTime) -> DateTime<Local> {
    DateTime::<Local>::from(st)
}

/// Media create date via nom-exif v3 (images + video/audio tracks).
fn try_media_creation(path: &Path, parser: &mut MediaParser) -> Option<DateTime<Local>> {
    let ms = match MediaSource::open(path) {
        Ok(ms) => ms,
        Err(e) => {
            eprintln!("[warn] cannot open as media {}: {e}", path.display());
            return None;
        }
    };

    match ms.kind() {
        MediaKind::Image => {
            let iter = match parser.parse_exif(ms) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("[warn] exif parse failed {}: {e}", path.display());
                    return None;
                }
            };
            let exif: Exif = iter.into();
            for tag in [
                ExifTag::DateTimeOriginal,
                ExifTag::CreateDate,
                ExifTag::ModifyDate,
            ] {
                if let Some(v) = exif.get(tag)
                    && let Some(dt) = entry_to_local(v)
                {
                    return Some(dt);
                }
            }
            None
        }
        MediaKind::Track => {
            let info = match parser.parse_track(ms) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("[warn] track parse failed {}: {e}", path.display());
                    return None;
                }
            };
            info.get(TrackInfoTag::CreateDate).and_then(entry_to_local)
        }
    }
}

/// Filesystem times when media metadata has nothing useful.
/// Use the earlier of birthtime and mtime -- copy-in birthtime is often
/// "today" while mtime still reflects the original content age.
pub fn try_fs_time(path: &Path) -> Option<DateTime<Local>> {
    let meta = std::fs::metadata(path).ok()?;
    let created = meta.created().ok();
    let modified = meta.modified().ok();
    match (created, modified) {
        (Some(c), Some(m)) => Some(system_time_to_local(c.min(m))),
        (Some(c), None) => Some(system_time_to_local(c)),
        (None, Some(m)) => Some(system_time_to_local(m)),
        (None, None) => None,
    }
}

/// Resolve a stamp for `path`.
///
/// Order:
/// 1. Media metadata (EXIF / track CreateDate)
/// 2. Filesystem `min(created, mtime)`
/// 3. Epoch fallback ([`EPOCH_STAMP`]) with a warning
pub fn get_creation_time(path: &Path, parser: &mut MediaParser) -> Result<Cow<'static, str>> {
    if let Some(dt) = try_media_creation(path, parser) {
        return Ok(Cow::Owned(format_dual(dt)));
    }

    if let Some(dt) = try_fs_time(path) {
        eprintln!("[warn] no media date for {}, using filesystem time", path.display());
        return Ok(Cow::Owned(format_dual(dt)));
    }

    eprintln!("[warn] no creation date for {}, using epoch fallback", path.display());
    Ok(Cow::Borrowed(EPOCH_STAMP))
}

/// Convenience for call sites / tests that don't already own a parser.
pub fn get_creation_time_oneshot(path: &Path) -> Result<Cow<'static, str>> {
    let mut parser = MediaParser::new();
    get_creation_time(path, &mut parser)
}

#[cfg(test)]
mod processing_tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn rename_from_stamp_prefixes() {
        let seps = Separators::default();
        let out = rename_from_stamp("vacation.mp4", "26/05/2022 14:40 02:40 PM", &seps).unwrap();
        assert_eq!(out, ConvertOutcome::Renamed("2022_05_26 14.40 02.40PM vacation.mp4".into()));
    }

    #[test]
    fn rename_from_stamp_skips_already_prefixed() {
        let seps = Separators::default();
        let name = "2022_05_26 14.40 02.40PM vacation.mp4";
        let out = rename_from_stamp(name, "26/05/2022 14:40 02:40 PM", &seps).unwrap();
        assert_eq!(out, ConvertOutcome::AlreadyPrefixed);
    }

    #[test]
    fn rename_from_stamp_date_only() {
        let seps = Separators::default();
        let out = rename_from_stamp("shot.png", "07/10/2023", &seps).unwrap();
        assert_eq!(out, ConvertOutcome::Renamed("2023_10_07 shot.png".into()));
    }

    #[test]
    fn rename_from_stamp_rejects_garbage() {
        let seps = Separators::default();
        assert!(rename_from_stamp("x.png", "not-a-date", &seps).is_err());
    }

    #[test]
    fn format_dual_has_am_pm_and_24h() {
        let dt = Local.with_ymd_and_hms(2022, 5, 26, 14, 40, 0).unwrap();
        let s = format_dual(dt);
        assert!(s.contains("PM") || s.contains("AM"), "{s}");
        assert!(s.contains("14:40"), "{s}");
    }

    #[test]
    fn try_fs_time_reads_mtime() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("t.png");
        std::fs::write(&path, b"x").unwrap();
        // pin mtime in the past
        let past = filetime::FileTime::from_unix_time(1_696_680_000, 0); // ~2023-10-07
        filetime::set_file_mtime(&path, past).unwrap();
        let dt = try_fs_time(&path).expect("fs time");
        assert_eq!(dt.format("%Y").to_string(), "2023");
    }
}
