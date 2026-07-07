use std::io::{Error, Read};

use crate::{cups_reader::CUPSReader, page::DecompressedPage, sync::RasterVersion};

/// represents the whole rasterized document sent by the CUPS
/// server. the version is derived from the sync bytes and
/// the pages store the list of pages which consist of a header
/// and the body with the pixel data
pub struct Raster<R: Read> {
    cups_reader: CUPSReader<R>,
    pub version: RasterVersion,
}

impl<R: Read> Raster<R> {
    pub fn new(reader: R) -> Self {
        let cups_reader = CUPSReader::from_reader(reader).unwrap();
        let version = cups_reader.raster_version.clone();

        Self {
            cups_reader,
            version,
        }
    }
}

impl<R: Read> Iterator for Raster<R> {
    type Item = Result<DecompressedPage, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match DecompressedPage::from_cups_reader(&mut self.cups_reader) {
            Ok(p) => Option::Some(Ok(p)),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}
