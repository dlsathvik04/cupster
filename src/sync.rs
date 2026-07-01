use crate::errors::ParsingError;

pub enum Endian {
    Big,
    Little,
}

pub enum RasterVersion {
    V1(Endian),
    V2(Endian),
    V3(Endian),
}

impl RasterVersion {
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
