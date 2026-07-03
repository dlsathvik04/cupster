use std::io::Read;

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
    pub coallate: u32,
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
    pub cups_marker_type: String,
    pub cups_rendering_intent: String,
    pub cups_page_size_name: String,
}

/// Defines the header structure of the
/// Version 3 of the CUPS raster format.
/// Version 3 header is the same as the version 2
pub struct V3Header {
    pub v2_header: V2Header,
}

pub enum PageHeader {
    V1(V1Header),
    V2(V2Header),
    V3(V3Header),
}

impl PageHeader {
    fn from_bytes(header_bytes: Vec<u8>, raster_version: RasterVersion) -> Self {
        match raster_version {
            RasterVersion::V1(endian) => todo!(),
            RasterVersion::V2(endian) => todo!(),
            RasterVersion::V3(endian) => todo!(),
        }
    }
}
