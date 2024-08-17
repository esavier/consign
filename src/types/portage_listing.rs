use std::path::PathBuf;

use crate::types::error::ConsignError;
use crate::types::portage_dir::PortageDir;

#[derive(Debug, Default)]
pub struct PortageListing {
    pub categories: Vec<PortageDir>,
}

impl PortageListing {
    pub fn scan_default_dir_path(&mut self) -> std::result::Result<(), ConsignError> {
        // default directories:
        const PORTAGE_PACKAGE_USE: &str = "/etc/portage/package.use";
        const PORTAGE_PACKAGE_MASK: &str = "/etc/portage/package.mask";
        const PORTAGE_PACKAGE_UNMASK: &str = "/etc/portage/package.unmask";
        const PORTAGE_PACKAGE_KEYWORDS: &str = "/etc/portage/package.keywords";
        const PORTAGE_PACKAGE_LICENSE: &str = "/etc/portage/package.license";

        let package_use_dir = PathBuf::from(PORTAGE_PACKAGE_USE);
        let package_mask_dir = PathBuf::from(PORTAGE_PACKAGE_MASK);
        let package_unmask_dir = PathBuf::from(PORTAGE_PACKAGE_UNMASK);
        let package_keywords_dir = PathBuf::from(PORTAGE_PACKAGE_KEYWORDS);
        let package_license_dir = PathBuf::from(PORTAGE_PACKAGE_LICENSE);

        _ = self.scan_dir_path(package_use_dir).map_err(|e| {
            tracing::error!("Failed to scan package.use directory: {:?}", e);
            e
        });
        _ = self.scan_dir_path(package_mask_dir).map_err(|e| {
            tracing::error!("Failed to scan package.mask directory: {:?}", e);
            e
        });
        _ = self.scan_dir_path(package_unmask_dir).map_err(|e| {
            tracing::error!("Failed to scan package.unmask directory: {:?}", e);
            e
        });
        _ = self.scan_dir_path(package_keywords_dir).map_err(|e| {
            tracing::error!("Failed to scan package.keywords directory: {:?}", e);
            e
        });
        _ = self.scan_dir_path(package_license_dir).map_err(|e| {
            tracing::error!("Failed to scan package.license directory: {:?}", e);
            e
        });

        Ok(())
    }

    // take:
    // - path
    // - portage dir type
    // scans the directory for files, i.e. "package.use/x11-base" and similar entries
    // returns a PortageDir object, and inserts it into the categories hashmap
    pub fn scan_dir_path(
        &mut self,
        path: PathBuf,
    ) -> std::result::Result<(), ConsignError> {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut portage_dir = PortageDir::new(name);
        portage_dir.scan_dir_for_files(&path)?;
        self.categories.push(portage_dir);
        Ok(())
    }
}
