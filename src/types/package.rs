pub use crate::types::{categories::PackageCategory, error::ConsignError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub category: Option<PackageCategory>,
    pub name: String,
    pub version: Option<String>,
    pub available_flags: Vec<String>,
    pub enabled_flags: Vec<String>,
    pub disabled_flags: Vec<String>,
}

impl Package {
    pub fn new(
        category: Option<PackageCategory>,
        name: String,
        version: Option<String>,
        available_flags: Vec<String>,
        enabled_flags: Vec<String>,
        disabled_flags: Vec<String>,
    ) -> Package {
        Package {
            category,
            name,
            version,
            available_flags,
            enabled_flags,
            disabled_flags,
        }
    }

    pub fn try_from_line(line: &str) -> std::result::Result<Package, ConsignError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ConsignError::GeneralFailure("Invalid package line".to_string()));
        }
        if parts[0].starts_with('#') {
            return Err(ConsignError::GeneralFailure("Comment line".to_string()));
        }

        println!("{:?}", parts);
        println!("trying to parse category from {}", parts[0]);

        let (category, name) = Self::maybe_category_from_line(parts[0]);
        let version = Self::maybe_version_from_line(parts[0]);
        let available_flags = parts[1..].iter().map(|s| s.to_string()).collect();
        Ok(Package::new(
            category,
            name,
            version,
            available_flags,
            Vec::new(),
            Vec::new(),
        ))
    }

    #[tracing::instrument(level = "debug", ret)]
    pub fn maybe_category_from_line(line: &str) -> (Option<PackageCategory>, String) {
        match line.split_once('/') {
            Some((category_str, name_str)) => {
                if name_str.is_empty() {
                    tracing::error!("Invalid package line, defaulting to {}", line);
                    return (None, line.to_string());
                } else {
                    tracing::error!("Parsed category: {} name: {}", category_str, name_str);
                    // TODO can not parse category if category is prefixed with >= or similar
                    // we need to distinguish between package definition and versioning

                    return (PackageCategory::try_from(category_str).ok(), name_str.to_string());
                }
            }
            None => (None, line.to_string()),
        }
    }

    #[tracing::instrument(level = "debug", ret)]
    pub fn maybe_version_from_line(line: &str) -> Option<String> {
        let mut current = line;
        loop {
            if let Some((_prefix, sufix)) = current.split_once('-') {
                current = sufix;
                // check if suffix starts with number
                if current.chars().next().unwrap().is_numeric() {
                    return Some(sufix.to_string());
                } else {
                    continue;
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_try_from_line_basic() {
        let line = "dev-lang/rust-bin rust-analyzer rust-src verify-sig rustfmt doc -icu";
        let package = Package::try_from_line(line).unwrap();

        assert_eq!(package.category, PackageCategory::try_from("dev-lang").ok());
        assert_eq!(package.name, "rust-bin");
        assert!(package.version.is_none());
        assert_eq!(
            package.available_flags,
            vec!["rust-analyzer", "rust-src", "verify-sig", "rustfmt", "doc", "-icu"]
        );
    }

    #[test]
    fn package_try_from_line_comment() {
        let line = "# dev-lang/rust-bin rust-analyzer rust-src verify-sig rustfmt doc -icu";
        let package = Package::try_from_line(line);
        assert!(package.is_err());
    }

    #[test]
    fn package_try_from_line_short() {
        let line = "dev-lang/rust-bin";
        let package = Package::try_from_line(line).unwrap();
        assert_eq!(package.category, PackageCategory::try_from("dev-lang").ok());
        assert_eq!(package.name, "rust-bin");
    }

    #[test]
    fn package_name_without_category() {
        let line = "rust-bin";
        let (category, name) = Package::maybe_category_from_line(line);
        assert_eq!(category, None);
        assert_eq!(name, "rust-bin");
    }

    #[test]
    fn package_version_parsing_complex() {
        let line = "media-gfx/exiv2-0.25-r2";
        let version = Package::maybe_version_from_line(line).unwrap();
        assert_eq!(version, "0.25-r2");
    }

    #[test]
    fn package_version_parsing_simple() {
        let line = "exiv2-0.25-r2";
        let version = Package::maybe_version_from_line(line).unwrap();
        assert_eq!(version, "0.25-r2");
    }
}
