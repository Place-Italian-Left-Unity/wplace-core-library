use std::io::Write;

use curl::easy::Handler;

pub mod color;
pub mod image_comparison;
pub mod image_data;
pub mod map_coords;
pub mod nominatim_data;
pub mod template_data;
pub mod tile_coords;
pub mod tile_downloader;

#[inline(always)]
pub fn convert_px_to_hours(px: u32) -> f64 {
    (px as f64) / 120.0
}

struct GenericBytes(Vec<u8>);
impl Handler for GenericBytes {
    fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
        self.0
            .write_all(data)
            .map_err(|_| curl::easy::WriteError::Pause)?;
        Ok(data.len())
    }
}

// struct GenericBytesCursor<'a>(&'a mut std::io::Cursor<Vec<u8>>);
// impl <'a>Handler for GenericBytesCursor<'a> {
//     fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
//         self.0.write_all(data);
//         Ok(data.len())
//     }
// }
