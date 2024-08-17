pub mod counting;
pub mod dirscan;

use crate::types::portage_listing::PortageListing;

#[tracing::instrument(level = "debug", ret)]
pub fn count_packages_in_listing(listing: &PortageListing) -> usize {
    let mut count = 0;
    for dir in &listing.categories {
        for file in &dir.filelist {
            count += file.packages.len();
        }
    }
    count
}
