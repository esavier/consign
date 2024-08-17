use types::portage_listing::PortageListing;

use crate::types::error;

pub use crate::types::error::ConsignError;
pub mod logging;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() -> std::result::Result<(), error::ConsignError> {
    logging::init_subscriber().map_err(|e| ConsignError::LoggingError(e.to_string()))?;

    let mut portage_listing = PortageListing::default();
    portage_listing.scan_default_dir_path()?;

    // dbg!(&portage_listing);
    // dbg!(utils::count_packages_in_listing(&portage_listing));

    // ============
    let mut package_listing = crate::types::package_listing::PackageListing::default();
    package_listing.scan_portage_listing(&portage_listing);
    package_listing.display();

    Ok(())
}
