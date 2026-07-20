mod crawlers;
mod folder_types;

use std::path::Path;

use crawlers::{crawl_dir_recursive, crawl_dir_recursive_par};
use eyre::Result;
pub use folder_types::{Folder, Folders, MediaType, ValidFileTypes};

/// How to walk the directory tree.
///
/// Convert from the CLI `--serial` flag via [`From<bool>`]:
/// `true` -> [`Serial`](CrawlType::Serial), `false` -> [`Parallel`](CrawlType::Parallel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CrawlType {
    Serial,
    #[default]
    Parallel,
}

impl From<bool> for CrawlType {
    /// `true` = serial crawl (matches `--serial`), `false` = parallel (default).
    #[inline]
    fn from(serial: bool) -> Self {
        if serial { Self::Serial } else { Self::Parallel }
    }
}

pub fn crawl_dir<P: AsRef<Path>>(root: P, crawl_type: CrawlType) -> Result<Folders> {
    let root = root.as_ref();
    let root_folder = match crawl_type {
        CrawlType::Serial => crawl_dir_recursive(root)?,
        CrawlType::Parallel => crawl_dir_recursive_par(root)?,
    };

    let mut folders = Folders::new();
    folders.add_folder(root_folder);
    Ok(folders)
}
