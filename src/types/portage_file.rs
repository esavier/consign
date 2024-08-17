use crate::types::package::Package;
use std::io::BufRead;

#[derive(Debug)]
pub struct PortageFile {
    pub path: std::path::PathBuf,
    pub packages: Vec<Package>,
}

impl PortageFile {
    pub fn new(path: std::path::PathBuf) -> PortageFile {
        PortageFile {
            path,
            packages: Vec::new(),
        }
    }
    pub fn scan_file_for_packages(&mut self) -> std::result::Result<(), crate::types::error::ConsignError> {
        let file = std::fs::File::open(&self.path)?;
        let reader = std::io::BufReader::new(file);
        let mut line_nr: usize = 0;
        for line in reader.lines() {
            line_nr += 1;
            let line = line?;
            match Package::try_from_line(&line) {
                Ok(package) => {
                    self.packages.push(package);
                }
                Err(_) => {
                    tracing::warn!("Failed to parse line: {}::>{}", line_nr, line);
                    continue;
                }
            }
        }
        Ok(())
    }
}
