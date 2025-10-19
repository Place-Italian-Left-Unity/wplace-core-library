pub mod color;
pub mod image_comparison;
pub mod image_data;
pub mod map_coords;
pub mod template_data;
pub mod tile_coords;

#[inline(always)]
pub fn convert_px_to_hours(px: u32) -> f64 {
    (px as f64) / 120.0
}
