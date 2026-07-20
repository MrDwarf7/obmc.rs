//! Organize media files by prefixing filenames with media creation dates.

pub mod cli;
pub mod folders;
pub mod prelude;
pub mod processing;
pub mod separators;
pub mod stamp_converter;

pub use folders::{CrawlType, Folder, Folders, crawl_dir};
pub use prelude::*;
pub use processing::{ConvertOutcome, process_folder, rename_from_stamp};
pub use separators::Separators;
pub use stamp_converter::flip_date_format;
