use std::io::{Read, Result};

use crate::{cups_reader::CUPSReader, page_header::PageHeader};

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
            PageHeader::V1(_) | PageHeader::V3(_) => reader.read_n_bytes(page_body_size)?,
            PageHeader::V2(_) => {
                let mut pixel_values: Vec<u8> = Vec::with_capacity(page_body_size);

                let unit_bytes = match cups_color_order {
                    0 => (base.cups_bits_per_pixel + 7).div_ceil(8),
                    _ => (base.cups_bits_per_color + 7).div_ceil(8),
                };

                while page_body_size > pixel_values.len() {
                    let line_rep_byte = reader.read_byte()? as u16 + 1;
                    let mut current_line: Vec<u8> = Vec::with_capacity(bytes_per_line);
                    while current_line.len() < bytes_per_line {
                        let control_byte = reader.read_byte()?;
                        if control_byte < 128 {
                            let color_repeat_count = control_byte + 1;
                            let color_bytes = reader.read_n_bytes(unit_bytes as usize)?;
                            for _ in 0..color_repeat_count {
                                current_line.extend_from_slice(&color_bytes);
                            }
                        } else {
                            let literal_unit_count = 257 - control_byte as u16;
                            let literal_bytes = reader
                                .read_n_bytes(literal_unit_count as usize * unit_bytes as usize)?;
                            current_line.extend_from_slice(&literal_bytes);
                        }
                    }
                    for _ in 0..line_rep_byte {
                        pixel_values.extend_from_slice(&current_line);
                    }
                }
                pixel_values
            }
        };

        Ok(Self {
            header,
            pixel_values,
        })
    }
}
