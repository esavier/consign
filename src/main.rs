use std::path::{Path, PathBuf};

use error::ConsignError;

pub mod error;
pub mod logging;

// hardcoded for now
pub const PORTAGE_DIRECTORY: &str = "/etc/portage";
pub const PORTAGE_PACKAGE_USE: &str = "/etc/portage/package.use";
pub const PORTAGE_PACKAGE_MASK: &str = "/etc/portage/package.mask";
pub const PORTAGE_PACKAGE_UNMASK: &str = "/etc/portage/package.unmask";
pub const PORTAGE_PACKAGE_KEYWORDS: &str = "/etc/portage/package.keywords";
pub const PORTAGE_PACKAGE_LICENSE: &str = "/etc/portage/package.license";

#[derive(Debug)]
pub struct PortageListing {
    pub root: PathBuf,
    pub categories: Vec<DirCategory>,
    pub available_categories: Vec<String>,
}

impl Default for PortageListing {
    fn default() -> PortageListing {
        PortageListing {
            root: PathBuf::from(PORTAGE_DIRECTORY),
            categories: Vec::new(),
            available_categories: vec![
                String::from(PORTAGE_PACKAGE_USE),
                String::from(PORTAGE_PACKAGE_MASK),
                String::from(PORTAGE_PACKAGE_KEYWORDS),
                String::from(PORTAGE_PACKAGE_LICENSE),
                String::from(PORTAGE_PACKAGE_UNMASK),
            ],
        }
    }
}

impl PortageListing {
    #[tracing::instrument(skip(self), level = "debug", ret)]
    pub async fn try_all_available_subdirs(&mut self) -> std::result::Result<(), error::ConsignError> {
        let root = Path::new(PORTAGE_DIRECTORY);
        // root directory must exist
        if !root.exists() {
            tracing::error!("Root directory does not exist: {:?}", root);
            return Err(error::ConsignError::FileNotFound(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )));
        };

        let mut successes: usize = 0;
        let mut tries: usize = 0;
        for each in self.available_categories.clone() {
            tries += 1;
            let mut dircat = DirCategory::new_from_path(Path::new(&each));
            match dircat.scan_subdirs(Path::new(&each)).await {
                Ok(_) => {
                    successes += 1;
                    self.categories.push(dircat);
                }
                Err(e) => {
                    tracing::error!("Error scanning subdirs: {:?}", e);
                    continue;
                }
            }
        }

        if successes == 0 {
            tracing::error!("No successful scans");
            return Err(error::ConsignError::FileNotFound(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )));
        } else if tries != successes {
            tracing::warn!("Not all scans were successful: {} / {}", successes, tries);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DirCategory {
    pub name: String,
    pub categories: Vec<PackageCategory>,
}

impl DirCategory {
    pub fn new(name: String) -> DirCategory {
        DirCategory {
            name,
            categories: Vec::new(),
        }
    }
    pub fn new_from_path(path: &Path) -> DirCategory {
        DirCategory {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            categories: Vec::new(),
        }
    }

    #[tracing::instrument(skip(self), level = "debug", ret)]
    pub async fn scan_subdirs(
        &mut self,
        path: &Path,
    ) -> std::result::Result<(), error::ConsignError> {
        let entries = std::fs::read_dir(path)?;
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_dir() {
                        continue;
                    } else {
                        let category = PackageCategory::try_new(path.clone())?;
                        self.categories.push(category);
                    }
                }
                Err(e) => {
                    tracing::error!("Error reading directory: {:?}", e);
                    continue;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct PackageCategory {
    pub name: String,
    pub path: std::path::PathBuf,
    pub lines: Vec<String>,
    // packages: Vec<Package>,
}

impl PackageCategory {
    pub fn try_new(path: std::path::PathBuf) -> std::result::Result<PackageCategory, ConsignError> {
        let name = match path.file_name() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => {
                return Err(error::ConsignError::FileNotFound(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                )))
            }
        };

        Ok(PackageCategory {
            name,
            path,
            lines: Vec::new(),
        })
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), error::ConsignError> {
    logging::init_subscriber().map_err(|e| ConsignError::LoggingError(e.to_string()))?;

    let mut portage_listing = PortageListing::default();
    portage_listing.try_all_available_subdirs().await?;
    dbg!(&portage_listing);
    Ok(())
}
