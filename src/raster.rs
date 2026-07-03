use crate::header::PageHeader;
use crate::sync::RasterVersion;

/// represents a single page in the whole data.
/// contains the raser format verson, a header and
/// the page data
struct Page {
    raster_version: RasterVersion,
    header: PageHeader,
}

/// represents the whole rasterized document sent by the CUPS
/// server. the version is derived from the sync bytes and
/// the pages store the list of pages which consist of a header
/// and the body with the pixel data
struct Raster {
    version: RasterVersion,
    pages: Vec<Page>,
}

impl Raster {}
