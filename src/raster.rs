enum ParsingError {
    InvalidSyncBytes,
}

enum Endian {
    Big,
    Little,
}

enum RasterVersion {
    V1(Endian),
    V2(Endian),
    V3(Endian),
}

impl RasterVersion {
    fn from_sync_strem(sync_bytes: &str) -> Result<Self, ParsingError> {
        match sync_bytes {
            "RaSt" => Ok(RasterVersion::V1(Endian::Big)),
            "tSaR" => Ok(RasterVersion::V1(Endian::Little)),
            "RaS2" => Ok(RasterVersion::V2(Endian::Big)),
            "2SaR" => Ok(RasterVersion::V2(Endian::Little)),
            "RaS3" => Ok(RasterVersion::V3(Endian::Big)),
            "3SaR" => Ok(RasterVersion::V3(Endian::Little)),
            _ => Err(ParsingError::InvalidSyncBytes),
        }
    }
}

struct V1Header {
    media_class: String,
    media_color: String,
    media_type: String,
    output_type: String,
    advance_distance: u32,
    advance_media: u32,
    coallate: u32,
    cut_media: u32,
    duplex: u32,
    hw_resolution_horizontal: u32,
    hw_resolution_vertical: u32,
    image_bounding_box_left: u32,
    image_bounding_box_bottom: u32,
    image_bounding_box_right: u32,
    image_bounding_box_top: u32,
    insert_sheet: u32,
    jog: u32,
    leading_edge: u32,
    margin_left_pts: u32,
    margin_bottom_pts: u32,
    manual_feed: u32,
    media_position: u32,
    media_weight: u32,
    mirror_print: u32,
    negative_print: u32,
    num_copies: u32,
    orientation: u32,
    output_face_up: u32,
    page_size_width: u32,
    page_size_length: u32,
    separations: u32,
    tray_switch: u32,
    tumble: u32,
    cups_width: u32,
    cups_height: u32,
    cups_media_type: u32,
    cups_bits_per_color: u32,
    cups_bits_per_pixel: u32,
    cups_bytes_per_line: u32,
    cups_color_order: u32,
    cups_color_space: u32,
    cups_compression: u32,
    cups_row_count: u32,
    cups_row_feed: u32,
    cups_row_step: u32,
}

struct V2Header {
    v1_header: V1Header,
    cups_num_colors: u32,
    cups_borderless_scaling_factor: f32,
    cups_page_size_width: u32,
    cups_page_size_length: u32,
    cups_imaging_box_left: u32,
    cups_imaging_box_bottom: u32,
    cups_imaging_box_right: u32,
    cups_imaging_box_top: u32,
    cups_integers: [u32; 16],
    cups_reals: [f32; 16],
    cups_strings: [u8; 1024],
    cups_marker_type: String,
    cups_rendering_intent: String,
    cups_page_size_name: String,
}

struct V3Header {
    v2_header: V2Header,
}

struct Raster {
    // header: Header,
}
