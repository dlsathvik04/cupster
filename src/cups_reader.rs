use crate::sync::{Endian, RasterVersion};
use std::io::{Error, Read, Result};

pub struct CUPSReader<R: Read> {
    reader: R,
    pub endian: Endian,
    pub raster_version: RasterVersion,
}

impl<R: Read> CUPSReader<R> {
    pub fn from_reader(mut reader: R) -> Result<Self> {
        let mut buff = [0u8; 4];
        reader.read_exact(&mut buff)?;
        let raster_version = match RasterVersion::from_sync_bytes(&buff) {
            Ok(rv) => rv,
            Err(_) => return Err(Error::other("Invalid Sync Bytes")),
        };
        let endian = match raster_version.clone() {
            RasterVersion::V1(endian) => endian,
            RasterVersion::V2(endian) => endian,
            RasterVersion::V3(endian) => endian,
        };

        Ok(CUPSReader {
            reader,
            endian,
            raster_version,
        })
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let mut buff = [0u8; 4];
        self.reader.read_exact(&mut buff)?;
        match self.endian {
            Endian::Big => Ok(u32::from_be_bytes(buff)),
            Endian::Little => Ok(u32::from_le_bytes(buff)),
        }
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let mut buff = [0u8; 4];
        self.reader.read_exact(&mut buff)?;
        match self.endian {
            Endian::Big => Ok(f32::from_be_bytes(buff)),
            Endian::Little => Ok(f32::from_le_bytes(buff)),
        }
    }

    pub fn read_bytes_64(&mut self) -> Result<[u8; 64]> {
        let mut buf: [u8; 64] = [0; 64];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn read_byte(&mut self) -> Result<u8> {
        let mut byte = [0u8];
        self.reader.read_exact(&mut byte)?;
        Ok(byte[0])
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buff: Vec<u8> = vec![0u8; n];
        self.reader.read_exact(&mut buff)?;
        Ok(buff)
    }
}
