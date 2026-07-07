use crate::errors::ParsingError;

/// Enum to reprenset the endianness of the source machin
/// will be derived from sync_bytes
#[derive(Clone)]
pub enum Endian {
    Big,
    Little,
}

/// Enum to reprenset the Raster Format version
/// the version may effect the header structure
#[derive(Clone)]
pub enum RasterVersion {
    V1(Endian),
    V2(Endian),
    V3(Endian),
}

impl RasterVersion {
    /// derives the raster version and the endianness of the fomat using the
    /// sync bytes (32bit)
    pub fn from_sync_bytes(sync_bytes: &[u8; 4]) -> Result<Self, ParsingError> {
        match sync_bytes {
            b"RaSt" => Ok(RasterVersion::V1(Endian::Big)),
            b"tSaR" => Ok(RasterVersion::V1(Endian::Little)),
            b"RaS2" => Ok(RasterVersion::V2(Endian::Big)),
            b"2SaR" => Ok(RasterVersion::V2(Endian::Little)),
            b"RaS3" => Ok(RasterVersion::V3(Endian::Big)),
            b"3SaR" => Ok(RasterVersion::V3(Endian::Little)),
            _ => Err(ParsingError::InvalidSyncBytes),
        }
    }
}
