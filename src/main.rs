mod folders;
mod prelude;
mod processing;
mod seperators;
mod stamp_converter;

pub use processing::process_folder;

pub use crate::prelude::*;
pub use crate::seperators::Seperators;

// TODO: Add a cli & add a proper logging system lol

// Apparently this can pull `media created at` from mp4 files
// https://github.com/alfg/mp4-rust

// There's also an ffmpeg crate lol, provides it as a hashmap and you can call .get("creation_time") on it

/// Global static used to store the data directory path.
/// This is dependent on whether or not we're in debug mode.
///
/// # Debug Mode
///
/// In debug mode, the data directory is set to `./data` relative to the current working directory.
///
/// # Release Mode
///
/// In release mode, the data directory is set to `./data` relative to the executable's location.
///
// pub static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(data_dir);
pub static mut DATA_DIR: LazyLock<PathBuf> = LazyLock::new(data_dir);

const FOLDER_VALID: [&str; 2] = ["-f", "--folder"];

fn folder_replace(args: Vec<String>) {
    let dir_index = args
        .iter()
        .position(|arg| FOLDER_VALID.contains(&arg.as_str()))
        .unwrap();
    unsafe {
        let folder_arg = args.get(dir_index + 1).unwrap_unchecked();
        let custom_path = PathBuf::from(folder_arg);
        std::mem::forget(std::mem::replace(&mut *DATA_DIR, custom_path));
    }
}

/// This function returns the path to the data directory based on the build configuration.
///
/// When in debug mode, it returns the `data` directory located in the current working directory.
/// When in release mode, it returns the `data` directory located in the same directory as
#[cfg(debug_assertions)]
pub fn data_dir() -> PathBuf {
    unsafe { std::env::current_dir().unwrap_unchecked().join("data") }
}

/// This function returns the path to the data directory based on the build configuration.
///
/// When in debug mode, it returns the `data` directory located in the current working directory.
/// When in release mode, it returns the `data` directory located in the same directory as
#[cfg(not(debug_assertions))]
pub fn data_dir() -> PathBuf {
    std::env::current_exe().unwrap().parent().unwrap().join("data")
}

const HELP_TEXT_VALID: [&str; 2] = ["-h", "--help"];

fn help_text() -> &'static str {
    r#"Usage: media_organizer [OPTIONS]
    A tool to organize media files into folders based on their creation date.
    -h, --help                  Print help information
    -d, --dry-run               Run the program without making any changes
    -f, --folder <FOLDER>       Specify the folder to organize (default is ./data)
    "#
}

const DRY_RUN_VALID: [&str; 3] = ["-d", "--dry-run", "--dry_run=true"];

/// Global static used to determine if the program should perform a dry run.
/// Default is true, meaning no files will be renamed.
pub static DRY_RUN_OPERATOR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub enum CrawlType {
    Serial,
    Parallel,
}

#[inline]
fn process_args() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.iter().any(|arg| HELP_TEXT_VALID.contains(&arg.as_str())) {
        println!("{}", help_text());
        std::process::exit(0);
    }

    if args.iter().any(|arg| DRY_RUN_VALID.contains(&arg.as_str())) {
        DRY_RUN_OPERATOR.store(true, std::sync::atomic::Ordering::Relaxed);
        println!("Dry run mode enabled. No files will be renamed.");
    }

    if args.iter().any(|arg| FOLDER_VALID.contains(&arg.as_str())) {
        folder_replace(args.clone());
    }
}

fn main() -> Result<()> {
    process_args();

    println!("Starting...");
    let start = std::time::Instant::now();

    let folders = folders::crawl_dir(unsafe { &*DATA_DIR }, CrawlType::Parallel)?;

    for folder in &folders.children {
        process_folder(folder);
    }

    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use super::*;
    use crate::processing::get_creation_time;

    static T_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(data_dir);
    const TEST_FILE: &str = "test.mp4";

    #[test]
    fn test_get_creation_time() {
        let test_file = T_DATA_DIR.join(TEST_FILE);
        let creation_time = get_creation_time(&test_file).unwrap();
        dbg!(&creation_time);

        assert!(creation_time.contains("AM") || creation_time.contains("PM"));
    }

    #[test]
    fn test_data_dir() {
        let data_dir = data_dir();
        assert!(data_dir.exists());
        assert!(data_dir.is_dir());

        let test_file = data_dir.join(TEST_FILE);
        assert!(test_file.exists());
        assert!(test_file.is_file());
    }
}
