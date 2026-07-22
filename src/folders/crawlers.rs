use std::path::Path;
use std::str::FromStr;

use eyre::{Context, Result};
use rayon::prelude::*;

use super::{Folder, MediaType, ValidFileTypes};

#[derive(Debug)]
enum PathType {
    File,
    Directory,
}

impl From<&Path> for PathType {
    #[inline]
    fn from(path: &Path) -> Self {
        if path.is_dir() {
            PathType::Directory
        } else {
            PathType::File
        }
    }
}

pub(super) fn crawl_dir_recursive(root: &Path) -> Result<Folder> {
    let mut folder = Folder::new(root.to_path_buf());

    let entries = std::fs::read_dir(root).wrap_err_with(|| format!("Failed to read directory: {}", root.display()))?;

    for entry_result in entries {
        let entry = entry_result.wrap_err("Failed to read directory entry")?;
        let path = entry.path();

        match PathType::from(path.as_path()) {
            PathType::Directory => {
                let child_folder = crawl_dir_recursive(&path)?;
                if !child_folder.children.is_empty() || !child_folder.children_files.is_empty() {
                    folder.add_folder(child_folder);
                }
            }
            PathType::File => {
                let Some(ext) = path.extension().and_then(|s| s.to_str()) else {
                    continue;
                };
                let Ok(media_type) = MediaType::from_str(ext) else {
                    continue;
                };
                folder.add_file(ValidFileTypes::new(path, media_type));
            }
        }
    }

    Ok(folder)
}

pub(super) fn crawl_dir_recursive_par(root: &Path) -> Result<Folder> {
    let entries: Vec<_> = std::fs::read_dir(root)
        .wrap_err_with(|| format!("Failed to read directory: {}", root.display()))?
        .collect::<Result<Vec<_>, _>>()
        .wrap_err("Failed to collect directory entries")?;

    let (dirs, files): (Vec<_>, Vec<_>) = entries.into_par_iter().partition_map(|entry| {
        let path = entry.path();
        if path.is_dir() {
            rayon::iter::Either::Left(path)
        } else {
            rayon::iter::Either::Right(path)
        }
    });

    let children_files: Vec<ValidFileTypes> = files
        .into_par_iter()
        .filter_map(|path| {
            let ext = path.extension()?.to_str()?;
            let media_type = MediaType::from_str(ext).ok()?;
            Some(ValidFileTypes::new(path, media_type))
        })
        .collect();

    let children: Vec<Folder> = dirs
        .into_par_iter()
        .filter_map(|dir_path| {
            match crawl_dir_recursive_par(&dir_path) {
                Ok(folder) if !folder.children.is_empty() || !folder.children_files.is_empty() => Some(folder),
                Ok(_) => None,
                Err(e) => {
                    eprintln!("[warn] error processing directory {}: {e}", dir_path.display());
                    None
                }
            }
        })
        .collect();

    Ok(Folder {
        path: root.to_path_buf(),
        children,
        children_files,
    })
}

#[cfg(test)]
mod crawler_tests {
    use tempfile::tempdir;

    use super::*;

    fn seed_tree(temp_dir_path: &Path) {
        std::fs::File::create(temp_dir_path.join("file1.mp4")).unwrap();
        std::fs::File::create(temp_dir_path.join("file2.mp4")).unwrap();
        let sub_dir = temp_dir_path.join("sub_dir");
        std::fs::create_dir(&sub_dir).unwrap();
        std::fs::File::create(sub_dir.join("sub_file.mp4")).unwrap();
    }

    #[test]
    fn test_crawl_dir_recursive() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        seed_tree(temp_dir_path);

        let folder = crawl_dir_recursive(temp_dir_path).unwrap();

        assert_eq!(folder.path, temp_dir_path);
        assert_eq!(folder.children.len(), 1);
        assert_eq!(folder.children_files.len(), 2);

        let sub_folder = &folder.children[0];
        assert_eq!(sub_folder.path, temp_dir_path.join("sub_dir"));
        assert_eq!(sub_folder.children.len(), 0);
        assert_eq!(sub_folder.children_files.len(), 1);
    }

    #[test]
    fn test_crawl_dir_recursive_par() {
        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        seed_tree(temp_dir_path);

        let folder = crawl_dir_recursive_par(temp_dir_path).unwrap();

        assert_eq!(folder.path, temp_dir_path);
        assert_eq!(folder.children.len(), 1);
        assert_eq!(folder.children_files.len(), 2);

        let sub_folder = &folder.children[0];
        assert_eq!(sub_folder.path, temp_dir_path.join("sub_dir"));
        assert_eq!(sub_folder.children.len(), 0);
        assert_eq!(sub_folder.children_files.len(), 1);
    }
}
