use std::io::{Read, Result};

use crate::{reader::CUPSReader, sync::RasterVersion};

/// Defines the structure of the header for the
/// Version 1 of the CUPS raster format
pub struct V1Header {
    pub media_class: [u8; 64],
    pub media_color: [u8; 64],
    pub media_type: [u8; 64],
    pub output_type: [u8; 64],
    pub advance_distance: u32,
    pub advance_media: u32,
    pub collate: u32,
    pub cut_media: u32,
    pub duplex: u32,
    pub hw_resolution_horizontal: u32,
    pub hw_resolution_vertical: u32,
    pub image_bounding_box_left: u32,
    pub image_bounding_box_bottom: u32,
    pub image_bounding_box_right: u32,
    pub image_bounding_box_top: u32,
    pub insert_sheet: u32,
    pub jog: u32,
    pub leading_edge: u32,
    pub margin_left_pts: u32,
    pub margin_bottom_pts: u32,
    pub manual_feed: u32,
    pub media_position: u32,
    pub media_weight: u32,
    pub mirror_print: u32,
    pub negative_print: u32,
    pub num_copies: u32,
    pub orientation: u32,
    pub output_face_up: u32,
    pub page_size_width: u32,
    pub page_size_length: u32,
    pub separations: u32,
    pub tray_switch: u32,
    pub tumble: u32,
    pub cups_width: u32,
    pub cups_height: u32,
    pub cups_media_type: u32,
    pub cups_bits_per_color: u32,
    pub cups_bits_per_pixel: u32,
    pub cups_bytes_per_line: u32,
    pub cups_color_order: u32,
    pub cups_color_space: u32,
    pub cups_compression: u32,
    pub cups_row_count: u32,
    pub cups_row_feed: u32,
    pub cups_row_step: u32,
}

impl V1Header {
    pub fn from_cups_reader<R: Read>(reader: &mut CUPSReader<R>) -> Result<Self> {
        Ok(Self {
            media_class: reader.read_bytes_64()?,
            media_color: reader.read_bytes_64()?,
            media_type: reader.read_bytes_64()?,
            output_type: reader.read_bytes_64()?,
            advance_distance: reader.read_u32()?,
            advance_media: reader.read_u32()?,
            collate: reader.read_u32()?,
            cut_media: reader.read_u32()?,
            duplex: reader.read_u32()?,
            hw_resolution_horizontal: reader.read_u32()?,
            hw_resolution_vertical: reader.read_u32()?,
            image_bounding_box_left: reader.read_u32()?,
            image_bounding_box_bottom: reader.read_u32()?,
            image_bounding_box_right: reader.read_u32()?,
            image_bounding_box_top: reader.read_u32()?,
            insert_sheet: reader.read_u32()?,
            jog: reader.read_u32()?,
            leading_edge: reader.read_u32()?,
            margin_left_pts: reader.read_u32()?,
            margin_bottom_pts: reader.read_u32()?,
            manual_feed: reader.read_u32()?,
            media_position: reader.read_u32()?,
            media_weight: reader.read_u32()?,
            mirror_print: reader.read_u32()?,
            negative_print: reader.read_u32()?,
            num_copies: reader.read_u32()?,
            orientation: reader.read_u32()?,
            output_face_up: reader.read_u32()?,
            page_size_width: reader.read_u32()?,
            page_size_length: reader.read_u32()?,
            separations: reader.read_u32()?,
            tray_switch: reader.read_u32()?,
            tumble: reader.read_u32()?,
            cups_width: reader.read_u32()?,
            cups_height: reader.read_u32()?,
            cups_media_type: reader.read_u32()?,
            cups_bits_per_color: reader.read_u32()?,
            cups_bits_per_pixel: reader.read_u32()?,
            cups_bytes_per_line: reader.read_u32()?,
            cups_color_order: reader.read_u32()?,
            cups_color_space: reader.read_u32()?,
            cups_compression: reader.read_u32()?,
            cups_row_count: reader.read_u32()?,
            cups_row_feed: reader.read_u32()?,
            cups_row_step: reader.read_u32()?,
        })
    }
}

/// Defines the header structure of the
/// Version 2 of the CUPS raster format.
/// Version 2 extnds Version1 with extra fields
pub struct V2Header {
    pub v1_header: V1Header,
    pub cups_num_colors: u32,
    pub cups_borderless_scaling_factor: f32,
    pub cups_page_size_width: u32,
    pub cups_page_size_length: u32,
    pub cups_imaging_bbox_left: f32,
    pub cups_imaging_bbox_bottom: f32,
    pub cups_imaging_bbox_right: f32,
    pub cups_imaging_bbox_top: f32,
    pub cups_integers: [u32; 16],
    pub cups_reals: [f32; 16],
    pub cups_strings: [[u8; 64]; 16],
    pub cups_marker_type: [u8; 64],
    pub cups_rendering_intent: [u8; 64],
    pub cups_page_size_name: [u8; 64],
}

impl V2Header {
    pub fn from_cups_reader<R: Read>(reader: &mut CUPSReader<R>) -> Result<Self> {
        let v1_header = V1Header::from_cups_reader(reader)?;
        let cups_num_colors = reader.read_u32()?;
        let cups_borderless_scaling_factor = reader.read_f32()?;
        let cups_page_size_width = reader.read_u32()?;
        let cups_page_size_length = reader.read_u32()?;
        let cups_imaging_bbox_left = reader.read_f32()?;
        let cups_imaging_bbox_bottom = reader.read_f32()?;
        let cups_imaging_bbox_right = reader.read_f32()?;
        let cups_imaging_bbox_top = reader.read_f32()?;
        let mut cups_integers = [0; 16];
        for i in &mut cups_integers {
            *i = reader.read_u32()?;
        }
        let mut cups_reals = [0f32; 16];
        for i in &mut cups_reals {
            *i = reader.read_f32()?;
        }

        let mut cups_strings = [[0u8; 64]; 16];
        for s in &mut cups_strings {
            *s = reader.read_bytes_64()?;
        }
        Ok(Self {
            v1_header,
            cups_num_colors,
            cups_borderless_scaling_factor,
            cups_page_size_width,
            cups_page_size_length,
            cups_imaging_bbox_left,
            cups_imaging_bbox_bottom,
            cups_imaging_bbox_right,
            cups_imaging_bbox_top,
            cups_integers,
            cups_reals,
            cups_strings,
            cups_marker_type: reader.read_bytes_64()?,
            cups_rendering_intent: reader.read_bytes_64()?,
            cups_page_size_name: reader.read_bytes_64()?,
        })
    }
}

/// Defines the header structure of the
/// Version 3 of the CUPS raster format.
/// Version 3 header is the same as the version 2
pub struct V3Header {
    pub v2_header: V2Header,
}

impl V3Header {
    pub fn from_cups_reader<R: Read>(reader: &mut CUPSReader<R>) -> Result<Self> {
        Ok(Self {
            v2_header: V2Header::from_cups_reader(reader)?,
        })
    }
}

pub enum PageHeader {
    V1(V1Header),
    V2(V2Header),
    V3(V3Header),
}

impl PageHeader {
    pub fn from_cups_reader<R: Read>(reader: &mut CUPSReader<R>) -> Result<Self> {
        match reader.raster_version {
            RasterVersion::V1(_) => Ok(PageHeader::V1(V1Header::from_cups_reader(reader)?)),
            RasterVersion::V2(_) => Ok(PageHeader::V2(V2Header::from_cups_reader(reader)?)),
            RasterVersion::V3(_) => Ok(PageHeader::V3(V3Header::from_cups_reader(reader)?)),
        }
    }
}
