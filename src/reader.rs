use crate::sync::Endian;
use std::io::{Read, Result};

pub struct CUPSReader<R: Read> {
    reader: R,
    endian: Endian,
}

impl<R: Read> CUPSReader<R> {
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

    pub fn read_str_64byte(&mut self) -> Result<[u8; 64]> {
        let mut buf: [u8; 64] = [0; 64];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}
