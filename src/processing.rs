use std::io::Write;
use std::path::Path;

pub use crate::DRY_RUN_OPERATOR; // Leave this as false to not accidentially rename a bunch of files lol
use crate::folders::Folder;
use crate::prelude::*;
use crate::{Seperators, stamp_converter};

fn write_handle<W: Write, S: AsRef<str>>(handle: &mut W, old: S, new: S, dry_run: bool) {
    if !dry_run {
        writeln!(handle, "{:<20} -> {:<20}", old.as_ref(), new.as_ref()).unwrap();
        return;
    }
    writeln!(handle, "[DRY RUN] | {:<20} -> {:<20}", old.as_ref(), new.as_ref()).unwrap();
}

pub fn process_folder(folder: &Folder) {
    let std_out = std::io::stdout();
    let mut handle = std_out.lock();

    let seps = Seperators::default();

    for file in &folder.children_files {
        let old_path = file.path.clone();
        let old_name = old_path.file_name().unwrap().to_str().unwrap();

        let new_name = format!(
            "{}, {old_name}",
            convert_name(&old_path, old_name, &seps).unwrap() // new_date_time
        );

        let new_path = old_path.with_file_name(new_name);
        let truncated_old = old_path.file_name().unwrap().to_str().unwrap();
        let truncated_new = new_path.file_name().unwrap().to_str().unwrap();

        if !DRY_RUN_OPERATOR.load(std::sync::atomic::Ordering::Relaxed) {
            write_handle(&mut handle, truncated_old, truncated_new, false);
            if new_path == old_path {
                // if program stops/crashes here, this ensures we don't run the same files a second time and append the date again
                continue;
            }
            // Rename the file
            std::fs::rename(&old_path, &new_path).unwrap();
            continue;
        }
        write_handle(&mut handle, old_path.to_str().unwrap(), truncated_new, true);
    }

    // Use recursion to process all subfolders
    for child_folder in &folder.children {
        process_folder(child_folder);
    }

    // These may actually be out of order due to the way stdout is buffered and the recursive nature of the function
    handle.flush().unwrap();
}

pub fn convert_name<P: AsRef<Path>>(old_path: &P, old_name: &str, seps: &Seperators) -> Result<String> {
    let creation_time = get_creation_time(&old_path).unwrap();
    let new_date = stamp_converter::flip_date_format(&creation_time, seps)?;
    let new_name = format!("{new_date} {old_name}");
    Ok(new_name)
}

/// The Windows implementation uses PowerShell to retrieve the actual creation time of the file.
/// This is a partial requirement due to how Windows handles file metadata.
/// There is potential here to implement via the windows crate, but that is a bit more involved.
/// I also doubt the performance difference would be noticeable for the intended use case.
///
/// # Platform-specific behavior
/// This function is only implemented for Windows systems. On non-Windows systems, a different
/// function should be used.
///
/// # Parameters
///
/// * `path`: A reference to a path-like object representing the file or directory whose creation
/// time is to be retrieved.
///
/// # Returns
/// * `Result<String>`: On success, returns a `String` representing the creation time formatted as
///
/// # Errors
///
/// * `eyre::Error`: If there is an error executing the PowerShell command or parsing its output.
///
// Is it pretty? No. Does it work? Yes.
// Should I be using c/c++/c# to do this? Probably.
// Will I? Maybe.
#[cfg(target_family = "windows")]
pub fn get_creation_time<P: AsRef<Path>>(path: &P) -> Result<String> {
    let cmd = std::process::Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "$File = Get-Item \"{}\";
            $ShellApplication = New-Object -ComObject Shell.Application;
            $ShellFolder = $ShellApplication.Namespace($File.Directory.FullName);
            $ShellFile = $ShellFolder.ParseName($File.Name);
            $v = $ShellFolder.GetDetailsOf($ShellFile, 208);
            Write-Host $v",
            path.as_ref().to_str().unwrap()
        ))
        .output()
        .wrap_err("Failed to execute powershell command")?;

    let output = String::from_utf8(cmd.stdout).wrap_err("Failed to convert output to string")?;
    let output = output.trim().to_string().replace("?", "");
    // Produces ----- `creation_time: "26/05/2022 12:40 AM"`

    // let as_systime = chrono::NaiveDateTime::parse_from_str(&output, "%d/%m/%Y %I:%M %p")
    //     .wrap_err("Failed to parse creation time")?;
    // println!("as_systime: {:?}", as_systime);

    Ok(output)
}

/// The UNIX implementation just uses the created metadata, which may not be the actual creation
/// time. This program is primarily intended for Windows use. This primarily functions
/// as a fill-in for testing and development while on UNIX systems.
///
///
/// # Platform-specific behavior
/// On Unix-like systems, the `created` metadata and is only designed to be used during
/// testing or development, as it may not accurately reflect the actual creation time of the file.
///
///
/// # Parameters
///
/// * `path`: A reference to a path-like object representing the file or directory whose creation time is to be retrieved.
///
/// # Returns
///
/// * `Result<String>`: On success, returns a `String` representing the creation time formatted as
///
/// # Errors
///
/// * `eyre::Error`: If there is an error retrieving the metadata or creation time.
///
#[inline]
#[cfg(target_family = "unix")]
pub fn get_creation_time<P: AsRef<Path>>(path: &P) -> Result<String> {
    use std::fs;

    use chrono::{DateTime, Local};

    let metadata = fs::metadata(path.as_ref())?;
    let created = metadata.created()?;

    let dt: DateTime<Local> = created.into();
    let formatted = dt.format("%d/%m/%Y %I:%M %p").to_string();

    Ok(formatted)
}
