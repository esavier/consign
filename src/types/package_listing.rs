use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::package::Package;
use crate::types::portage_listing::PortageListing;

use super::categories::PackageCategory;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageIdentifier {
    pub path: PathBuf,
    pub name: String,
}

impl PackageIdentifier {
    pub fn new(
        path: PathBuf,
        name: String,
    ) -> PackageIdentifier {
        PackageIdentifier { path, name }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PackageListing {
    pub packages: HashMap<PackageIdentifier, Package>,
}

impl PackageListing {
    pub fn display(&self) {
        for (key, value) in &self.packages {
            println!("{:40} | {:50} | {}", key.name, key.path.to_string_lossy(), value.name);
        }
    }

    pub fn scan_portage_listing(
        &mut self,
        portage_listing: &PortageListing,
    ) {
        for category in &portage_listing.categories {
            for file in &category.filelist {
                for package in &file.packages {
                    let category: PackageCategory = match package.category {
                        Some(category) => category,
                        None => PackageCategory::Unknown,
                    };
                    let full_name = format!("{}/{}", category, package.name);
                    self.packages
                        .insert(PackageIdentifier::new(file.path.clone(), full_name), package.clone());
                }
            }
        }
    }
}
