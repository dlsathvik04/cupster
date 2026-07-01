use crate::pageheader::PageHeader;
use crate::sync::RasterVersion;

struct Page {
    raster_version: RasterVersion,
    header: PageHeader,
}

struct Raster {
    version: RasterVersion,
    pages: Vec<Page>,
}
