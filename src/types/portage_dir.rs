use std::path::PathBuf;

use super::portage_file::PortageFile;

#[derive(Debug)]
pub struct PortageDir {
    pub dirname: String,
    pub path: std::path::PathBuf,
    pub filelist: Vec<PortageFile>,
}

impl PortageDir {
    pub fn new(name: String) -> PortageDir {
        PortageDir {
            dirname: name,
            path: std::path::PathBuf::new(),
            filelist: Vec::new(),
        }
    }

    pub fn scan_dir_for_files(
        &mut self,
        path: &std::path::Path,
    ) -> std::result::Result<(), crate::types::error::ConsignError> {
        let files = crate::utils::dirscan::scan_dir(path, crate::utils::dirscan::ScanFilter::FilesOnly)?;
        for file in files {
            let mut portage_file = PortageFile::new(PathBuf::from(file.as_path()));
            portage_file.scan_file_for_packages()?;
            self.filelist.push(portage_file);
        }
        Ok(())
    }
}
