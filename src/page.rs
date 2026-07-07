use std::io::{Read, Result};

use crate::{cups_reader::CUPSReader, page, page_header::PageHeader};

/// represents a single page in the whole data.
/// contains the raser format verson, a header and
/// the page data
pub struct DecompressedPage {
    pub header: PageHeader,
    pub pixel_values: Vec<u8>,
}

impl DecompressedPage {
    pub fn from_cups_reader<R: Read>(reader: &mut CUPSReader<R>) -> Result<Self> {
        let header = PageHeader::from_cups_reader(reader)?;

        let base = header.v1_spec();
        let bytes_per_line = base.cups_bytes_per_line as usize;
        let cups_height = base.cups_height as usize;
        let cups_color_order = base.cups_color_order as usize;

        let num_colors = match &header {
            PageHeader::V1(_) => 1,
            PageHeader::V2(v2_header) | PageHeader::V3(v2_header) => v2_header.cups_num_colors,
        } as usize;

        let page_body_size = match cups_color_order {
            0 => cups_height * bytes_per_line,
            1 | 2 => num_colors * cups_height * bytes_per_line,
            _ => bytes_per_line * cups_height,
        };

        let pixel_values: Vec<u8> = match &header {
            PageHeader::V1(_) | PageHeader::V3(_) => {
                reader.read_n_bytes(page_body_size as usize)?
            }
            PageHeader::V2(_) => {
                let mut pixel_values: Vec<u8> = Vec::with_capacity(page_body_size as usize);

                while pixel_values.len() < pixel_values.capacity() {
                    let repeat_byte = reader.read_byte()?;
                    if repeat_byte < 128 {
                    } else {
                    }
                }

                // TODO : Implement the decompression algorithm

                pixel_values
            }
        };

        Ok(Self {
            header,
            pixel_values,
        })
    }
}
