use crate::{page::DecompressedPage, sync::RasterVersion};

/// represents the whole rasterized document sent by the CUPS
/// server. the version is derived from the sync bytes and
/// the pages store the list of pages which consist of a header
/// and the body with the pixel data
struct Raster {
    version: RasterVersion,
    pages: Vec<DecompressedPage>,
}

impl Raster {}
