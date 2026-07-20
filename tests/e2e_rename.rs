//! End-to-end: crawl a temp tree, apply renames via filesystem mtimes, re-run is idempotent.

use std::fs;
use std::path::Path;
use std::process::Command;

use obmc::folders::{CrawlType, crawl_dir};
use obmc::{ConvertOutcome, Separators, process_folder, rename_from_stamp};
use tempfile::TempDir;

fn collect_names(dir: &Path) -> Vec<String> {
    let mut names: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| {
            let e = e.ok()?;
            (e.file_type().ok()?.is_file()).then(|| e.file_name().into_string().ok())
        })
        .flatten()
        .collect();
    names.sort();
    names
}

/// Process all folders in a crawl result.
fn process_all(folders: &obmc::folders::Folders, dry_run: bool) {
    for folder in &folders.children {
        process_folder(folder, dry_run);
    }
}

/// Count total files in a crawl result.
fn count_files(folder: &obmc::folders::Folder) -> usize {
    let mut n = folder.children_files.len();
    for child in &folder.children {
        n += count_files(child);
    }
    n
}

fn temp_rename_tree() -> (TempDir, std::path::PathBuf, std::path::PathBuf) {
    let root = TempDir::new().unwrap();
    let nested = root.path().join("nested");
    fs::create_dir(&nested).unwrap();

    let png_sig = b"\x89PNG\r\n\x1a\n";
    let a = root.path().join("alpha.png");
    let b = nested.join("beta.png");
    fs::write(&a, png_sig).unwrap();
    fs::write(&b, png_sig).unwrap();

    let _ = Command::new("touch")
        .args(["-d", "2023-10-07 15:24:00", a.to_str().unwrap()])
        .status();
    let _ = Command::new("touch")
        .args(["-d", "2026-03-01 23:35:00", b.to_str().unwrap()])
        .status();

    (root, a, b)
}

#[test]
fn e2e_crawl_rename_and_idempotent_skip() {
    let (root, _a, _b) = temp_rename_tree();
    let top = crawl_dir(root.path(), CrawlType::Serial).unwrap();

    assert_eq!(top.children[0].children_files.len(), 1, "root file count");
    assert_eq!(top.children.len(), 1, "nested dir");
    let sub = &top.children[0];
    assert_eq!(sub.children.len(), 1, "sub-dir");

    // apply
    process_all(&top, false);

    // verify names after rename
    let names = collect_names(root.path());
    assert!(
        names
            .iter()
            .any(|n| n.starts_with("2023_10_07") && n.ends_with("alpha.png")),
        "alpha renamed: {names:?}"
    );
    let sub_names = collect_names(&root.path().join("nested"));
    assert!(
        sub_names
            .iter()
            .any(|n| n.starts_with("2026_03_01") && n.ends_with("beta.png")),
        "beta renamed: {sub_names:?}"
    );

    // idempotent second pass
    process_all(&top, false);
    let names2 = collect_names(root.path());
    assert_eq!(names, names2, "second pass must not double-prefix");
    let sub_names2 = collect_names(&root.path().join("nested"));
    assert_eq!(sub_names, sub_names2, "second pass must not double-prefix nested");
}

#[test]
fn e2e_parallel_crawl_matches_serial_counts() {
    let root = TempDir::new().unwrap();
    fs::create_dir(root.path().join("sub")).unwrap();
    fs::write(root.path().join("a.mp4"), b"x").unwrap();
    fs::write(root.path().join("sub/b.mp4"), b"x").unwrap();
    fs::write(root.path().join("ignore.txt"), b"nope").unwrap();

    let serial = crawl_dir(root.path(), CrawlType::Serial).unwrap();
    let parallel = crawl_dir(root.path(), CrawlType::Parallel).unwrap();

    assert_eq!(count_files(&serial.children[0]), 2, "serial file count");
    assert_eq!(
        count_files(&serial.children[0]),
        count_files(&parallel.children[0]),
        "serial vs parallel file count"
    );
    assert_eq!(serial.children.len(), 1);
    assert_eq!(parallel.children.len(), 1);
}

#[test]
fn e2e_dry_run_does_not_touch_disk() {
    let root = TempDir::new().unwrap();
    let f = root.path().join("keep.png");
    fs::write(&f, b"\x89PNG\r\n\x1a\n").unwrap();
    let _ = Command::new("touch")
        .args(["-d", "2024-01-15 10:00:00", f.to_str().unwrap()])
        .status();

    let top = crawl_dir(root.path(), CrawlType::Serial).unwrap();
    process_all(&top, true); // dry run

    assert!(f.exists(), "dry-run must leave original path");
    let names = collect_names(root.path());
    assert_eq!(names, vec!["keep.png"]);
}

#[test]
fn pure_rename_api_round_trip_prefix() {
    let seps = Separators::default();
    let stamp = "15/01/2024 10:00 10:00 AM";
    let once = rename_from_stamp("keep.png", stamp, &seps).unwrap();
    assert!(matches!(once, ConvertOutcome::Renamed(_)), "expected rename");
    if let ConvertOutcome::Renamed(ref n) = once {
        let twice = rename_from_stamp(n, stamp, &seps).unwrap();
        assert_eq!(twice, ConvertOutcome::AlreadyPrefixed);
    } else {
        panic!("expected rename");
    }
}
