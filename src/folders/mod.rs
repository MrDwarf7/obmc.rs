mod crawlers;
mod folder_types;

use std::path::Path;

// For internal use by crawl_dir
use crawlers::{crawl_dir_recursive, crawl_dir_recursive_par};
use eyre::Result;
pub use folder_types::{Folder, Folders, ValidFileTypes, VideoType};

use crate::CrawlType;

// Could hand in an enum of 'CrawlType' to determine what type to do, recursive or non-recursive // parallel or serial
pub fn crawl_dir<P: AsRef<Path>>(root: &P, crawl_type: CrawlType) -> Result<Folders> {
    let root_folder = match crawl_type {
        CrawlType::Serial => crawl_dir_recursive(root)?,
        CrawlType::Parallel => crawl_dir_recursive_par(root)?,
    };

    let mut folders = Folders::new();
    folders.add_folder(root_folder);

    Ok(folders)
}
