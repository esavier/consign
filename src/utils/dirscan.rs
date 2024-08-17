use crate::types::error::ConsignError;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub enum ScanFilter {
    FilesOnly,
    DirectoriesOnly,
    All,
}

pub fn scan_dir(
    path: &Path,
    filter: ScanFilter,
) -> std::result::Result<Vec<PathBuf>, ConsignError> {
    let mut files: Vec<PathBuf> = Vec::new();

    if !path.is_dir() {
        return Err(ConsignError::GeneralFailure("Path is not a directory".to_string()));
    }
    let entries = std::fs::read_dir(path)?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    if filter == ScanFilter::FilesOnly {
                        continue;
                    }
                    files.push(path);
                } else {
                    if filter == ScanFilter::DirectoriesOnly {
                        continue;
                    }
                    files.push(path);
                }
            }
            Err(e) => {
                tracing::error!("Error reading directory: {:?}", e);
                continue;
            }
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_dir_files_only() {
        // create temporary directory
        let random_dir_name = format!("/tmp/consign/{}", uuid::Uuid::new_v4());
        let path = Path::new(&random_dir_name);
        std::fs::create_dir_all(path).unwrap();

        // create temporary files
        let file1 = path.join("file1");
        let file2 = path.join("file2");
        let file3 = path.join("file3");
        std::fs::File::create(&file1).unwrap();
        std::fs::File::create(&file2).unwrap();
        std::fs::File::create(&file3).unwrap();

        let files = scan_dir(path, ScanFilter::FilesOnly).unwrap();
        assert!(files.len() == 3);
        let mut results: usize = 0;
        for each in files {
            let entry = PathBuf::from(&each);
            assert!(entry.is_file());
            results += 1;
        }
        assert!(results == 3);
    }

    #[test]
    fn test_scan_dir_dir_only() {
        // create temporary directory
        let random_dir_name = format!("/tmp/consign/{}", uuid::Uuid::new_v4());
        let path = Path::new(&random_dir_name);
        std::fs::create_dir_all(path).unwrap();

        // create temporary dirs
        let dir1 = path.join("dir1");
        let dir2 = path.join("dir2");
        let dir3 = path.join("dir3");

        std::fs::create_dir(&dir1).unwrap();
        std::fs::create_dir(&dir2).unwrap();
        std::fs::create_dir(&dir3).unwrap();

        let files = scan_dir(path, ScanFilter::DirectoriesOnly).unwrap();
        assert!(files.len() == 3);
        let mut results: usize = 0;
        for each in files {
            let entry = PathBuf::from(&each);
            assert!(entry.is_dir());
            results += 1;
        }
        assert!(results == 3);
    }

    #[test]
    fn test_scan_dir_all() {
        // create temporary directory
        let random_dir_name = format!("/tmp/consign/{}", uuid::Uuid::new_v4());
        let path = Path::new(&random_dir_name);
        std::fs::create_dir_all(path).unwrap();

        // create temporary files
        let file1 = path.join("file1");
        let file2 = path.join("file2");
        let file3 = path.join("file3");
        let dir1 = path.join("dir1");
        let dir2 = path.join("dir2");
        let dir3 = path.join("dir3");
        std::fs::File::create(&file1).unwrap();
        std::fs::File::create(&file2).unwrap();
        std::fs::File::create(&file3).unwrap();
        std::fs::create_dir(&dir1).unwrap();
        std::fs::create_dir(&dir2).unwrap();
        std::fs::create_dir(&dir3).unwrap();

        let files = scan_dir(path, ScanFilter::All).unwrap();
        assert!(files.len() == 6);
        let mut results: usize = 0;
        for each in files {
            let entry = PathBuf::from(&each);
            assert!(entry.exists());
            results += 1;
        }
        assert!(results == 6);
    }
}
