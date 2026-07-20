use std::path::PathBuf;
use std::str::FromStr;

use eyre::Result;

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
    pub path:      PathBuf,
    pub type_data: MediaType,
}

impl ValidFileTypes {
    pub fn new(path: PathBuf, type_data: MediaType) -> Self {
        ValidFileTypes { path, type_data }
    }
}

#[derive(Debug)]
pub enum MediaType {
    Video(VideoType),
    Image(ImageType),
}

impl FromStr for MediaType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(video_type) = VideoType::from_str(s) {
            return Ok(MediaType::Video(video_type));
        }
        if let Ok(image_type) = ImageType::from_str(s) {
            return Ok(MediaType::Image(image_type));
        }
        Err(eyre::eyre!("Invalid media type: {s}"))
    }
}

#[derive(Debug)]
pub enum ImageType {
    Jpeg,
    Png,
    Heic,
}

impl FromStr for ImageType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(ImageType::Jpeg),
            "png" => Ok(ImageType::Png),
            "heic" => Ok(ImageType::Heic),
            _ => Err(eyre::eyre!("Invalid image type: {s}")),
        }
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
            _ => Err(eyre::eyre!("Invalid video type: {s}")),
        }
    }
}
