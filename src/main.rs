use obmc::cli::Cli;
use obmc::prelude::*;
use obmc::{folders, process_folder};

fn main() -> Result<()> {
    let cli = Cli::new();

    let data_dir = cli.folder.clone().unwrap_or_else(default_data_dir);

    if cli.dry_run {
        println!("Dry run mode enabled. No files will be renamed.");
    }

    if !data_dir.exists() {
        eyre::bail!("data directory does not exist: {}", data_dir.display());
    }

    println!("Starting in {}...", data_dir.display());
    let start = std::time::Instant::now();

    let folders = folders::crawl_dir(&data_dir, cli.crawl_type())?;

    // Folders wraps the root as its single child entry.
    for folder in &folders.children {
        process_folder(folder, cli.dry_run);
    }

    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
}
