use std::path::PathBuf;
use std::str::FromStr;

use eyre::Result;

// Hidden from exterrnally - access the exposed fn crawl_dir, not the internal implementation

#[derive(Debug, Default)]
pub struct Folders {
    pub children: Vec<Folder>,
}

impl Folders {
    #[rustfmt::skip]
    pub fn new() -> Self { Self::default() }

    pub fn add_folder(&mut self, folder: Folder) {
        self.children.push(folder);
    }
}

#[derive(Debug, Default)]
pub struct Folder {
    pub path:           PathBuf,
    pub children:       Vec<Folder>,
    pub children_files: Vec<ValidFileTypes>,
}

impl Folder {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.children.push(folder);
    }

    pub fn add_file(&mut self, file: ValidFileTypes) {
        self.children_files.push(file);
    }
}

#[derive(Debug)]
pub struct ValidFileTypes {
    pub path:  PathBuf,
    pub video: VideoType,
}

impl ValidFileTypes {
    pub fn new(path: PathBuf, video: VideoType) -> Self {
        ValidFileTypes { path, video }
    }
}

#[derive(Debug)]
pub enum VideoType {
    Mp4,
    Mov,
}

impl FromStr for VideoType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "mp4" => Ok(VideoType::Mp4),
            "mov" => Ok(VideoType::Mov),
            _ => Err(eyre::eyre!("Invalid video type: {}", s)),
        }
    }
}
