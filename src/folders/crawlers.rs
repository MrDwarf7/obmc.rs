use std::path::Path;
use std::str::FromStr;

use eyre::{Context, Result};
use rayon::prelude::*;

use super::{Folder, ValidFileTypes, VideoType};

#[derive(Debug)]
enum PathType {
    File,
    Directory,
}

impl<P> From<P> for PathType
where
    P: Clone + AsRef<Path>,
{
    #[inline]
    fn from(path: P) -> Self {
        let path = path.as_ref();
        if path.is_dir() {
            PathType::Directory
        } else {
            PathType::File
        }
    }
}

pub(super) fn crawl_dir_recursive<P: AsRef<Path>>(root: &P) -> Result<Folder> {
    let mut folder = Folder::new(root.as_ref().to_path_buf());

    let entries = std::fs::read_dir(root).wrap_err("Failed to read directory")?;

    for entry_result in entries {
        let entry = entry_result.wrap_err("Failed to read entry")?;
        let path = entry.path();

        let path_type = PathType::from(&path.clone());

        match path_type {
            PathType::Directory => {
                let child_folder = crawl_dir_recursive(&path)?;

                // Only add the folder if it contains files or subfolders
                if !child_folder.children.is_empty() || !child_folder.children_files.is_empty() {
                    folder.add_folder(child_folder);
                }
            }

            PathType::File => {
                let ext = match path.extension().and_then(|s| s.to_str()) {
                    Some(ext) => ext,
                    // Skip files without an extension or invalid UTF-8
                    None => continue,
                };

                let video_type = match VideoType::from_str(ext) {
                    Ok(video_type) => video_type,
                    // Skip invalid video types
                    Err(_) => continue,
                };

                let file = ValidFileTypes::new(path, video_type);
                folder.add_file(file);
            }
        }
    }

    Ok(folder)
}

pub fn crawl_dir_recursive_par<P: AsRef<Path>>(root: &P) -> Result<Folder> {
    let entries: Vec<_> = std::fs::read_dir(root.as_ref())
        .wrap_err_with(|| format!("Failed to read directory: {:?}", root.as_ref()))?
        .collect::<Result<Vec<_>, _>>()
        .wrap_err("Failed to collect directory entries")?;

    // Separate entries into directories and files
    let (dirs, files): (Vec<_>, Vec<_>) = entries.into_par_iter().partition_map(|entry| {
        let path = entry.path();
        if path.is_dir() {
            rayon::iter::Either::Left(path)
        } else {
            rayon::iter::Either::Right(path)
        }
    });

    // Process files in parallel
    let children_files: Vec<ValidFileTypes> = files
        .into_par_iter()
        .filter_map(|path| {
            // Get the extension
            let ext = path.extension()?.to_str()?;
            let video_type = VideoType::from_str(ext).ok()?;
            Some(ValidFileTypes::new(path, video_type))
        })
        .collect();

    // Process directories in parallel
    let children: Vec<Folder> = dirs
        .into_par_iter()
        .filter_map(|dir_path| {
            match crawl_dir_recursive_par(&dir_path) {
                Ok(folder) => {
                    // Only include non-empty folders
                    if !folder.children.is_empty() || !folder.children_files.is_empty() {
                        Some(folder)
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Error processing directory {:?}: {}", dir_path, e);
                    None
                }
            }
        })
        .collect();

    Ok(Folder {
        path: root.as_ref().to_path_buf(),
        children,
        children_files,
    })
}

#[cfg(test)]
mod crawler_tests {

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_crawl_dir_recursive() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        println!("temp_dir_path: {:?}", temp_dir_path);

        let file1 = temp_dir_path.join("file1.mp4");
        std::fs::File::create(&file1).unwrap();

        let file2 = temp_dir_path.join("file2.mp4");
        std::fs::File::create(&file2).unwrap();

        let sub_dir = temp_dir_path.join("sub_dir");
        std::fs::create_dir(&sub_dir).unwrap();

        let sub_file = sub_dir.join("sub_file.mp4");
        std::fs::File::create(&sub_file).unwrap();

        let folder = crawl_dir_recursive(&temp_dir_path).unwrap();

        assert_eq!(folder.path, temp_dir_path);
        assert_eq!(folder.children.len(), 1);
        assert_eq!(folder.children_files.len(), 2);

        let sub_folder = &folder.children[0];
        assert_eq!(sub_folder.path, sub_dir);
        assert_eq!(sub_folder.children.len(), 0);
        assert_eq!(sub_folder.children_files.len(), 1);
    }

    #[test]
    fn test_crawl_dir_recursive_par() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        println!("temp_dir_path: {:?}", temp_dir_path);

        let file1 = temp_dir_path.join("file1.mp4");
        std::fs::File::create(&file1).unwrap();

        let file2 = temp_dir_path.join("file2.mp4");
        std::fs::File::create(&file2).unwrap();

        let sub_dir = temp_dir_path.join("sub_dir");
        std::fs::create_dir(&sub_dir).unwrap();

        let sub_file = sub_dir.join("sub_file.mp4");
        std::fs::File::create(&sub_file).unwrap();

        let folder = crawl_dir_recursive_par(&temp_dir_path).unwrap();

        assert_eq!(folder.path, temp_dir_path);
        assert_eq!(folder.children.len(), 1);
        assert_eq!(folder.children_files.len(), 2);

        let sub_folder = &folder.children[0];
        assert_eq!(sub_folder.path, sub_dir);
        assert_eq!(sub_folder.children.len(), 0);
        assert_eq!(sub_folder.children_files.len(), 1);
    }
}
