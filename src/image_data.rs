use std::{collections::HashMap, fmt::Display, rc::Rc};

use image::{DynamicImage, GenericImageView, ImageBuffer, ImageReader, Rgba};

use crate::{
    color::Color, convert_px_to_hours, tile_coords::TileCoords,
    tile_downloader::TileDownloader,
};

/// Currently only supports PNG
pub struct ImageData {
    /// The parsed image
    pub(crate) image: image::DynamicImage,
    /// The parsed image's width
    pub(crate) width: u32,
    /// The parsed image's height
    pub(crate) height: u32,
    /// How many pixels there are of each color
    color_counts: HashMap<Color, u32>,
}

#[derive(Debug)]
pub enum ImageDataError {
    ImageError(image::error::ImageError),
    IoError(std::io::Error),
    InvalidWidth,
    InvalidHeight,
    InvalidColor { x: u32, y: u32, rgba: [u8; 4] },
}

impl Display for ImageDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWidth => write!(f, "Width is 0"),
            Self::InvalidHeight => write!(f, "Height is 0"),
            Self::ImageError(e) => write!(f, "Image Error: {e}"),
            Self::IoError(e) => write!(f, "I/O Error: {e}"),
            Self::InvalidColor {
                x,
                y,
                rgba: [r, g, b, a],
            } => write!(f, "Invalid color [{r},{g},{b},{a}] at {x}, {y}"),
        }
    }
}
impl std::error::Error for ImageDataError {}

pub trait IntoImageForImageData {
    fn into_image_for_image_data(self) -> Result<DynamicImage, ImageDataError>;
}

impl IntoImageForImageData for &[u8] {
    fn into_image_for_image_data(self) -> Result<DynamicImage, ImageDataError> {
        image::load_from_memory_with_format(self, image::ImageFormat::Png)
            .map_err(ImageDataError::ImageError)
    }
}

impl IntoImageForImageData for &std::path::Path {
    fn into_image_for_image_data(self) -> Result<DynamicImage, ImageDataError> {
        let mut reader = ImageReader::open(self).map_err(ImageDataError::IoError)?;
        reader.set_format(image::ImageFormat::Png);
        reader.decode().map_err(ImageDataError::ImageError)
    }
}

impl IntoImageForImageData for DynamicImage {
    fn into_image_for_image_data(self) -> Result<DynamicImage, ImageDataError> {
        Ok(self)
    }
}

impl IntoImageForImageData for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn into_image_for_image_data(self) -> Result<DynamicImage, ImageDataError> {
        Ok(DynamicImage::ImageRgba8(self))
    }
}

impl ImageData {
    pub fn new<R: IntoImageForImageData>(into_image_type: R) -> Result<Self, ImageDataError> {
        let image = into_image_type.into_image_for_image_data()?;

        let width = image.width();
        if width == 0 {
            return Err(ImageDataError::InvalidWidth);
        }

        let height = image.height();
        if height == 0 {
            return Err(ImageDataError::InvalidHeight);
        }

        let mut color_counts = HashMap::new();

        for (x, y, pixel) in image.pixels() {
            let rgba = pixel.0;

            if let [_, _, _, 0] = rgba {
                // Actually transparent pixels represent any color
                continue;
            };

            let color = Color::try_from(rgba);
            match color {
                Err(_) => return Err(ImageDataError::InvalidColor { x, y, rgba }),
                Ok(c) => match color_counts.get_mut(&c) {
                    Some(v) => *v += 1,
                    None => {
                        let _ = color_counts.insert(c, 1);
                    }
                },
            }
        }

        Ok(Self {
            image,
            width,
            height,
            color_counts,
        })
    }

    pub fn from_site_coords(
        top_left_corner: &TileCoords,
        width: u16,
        height: u16,
    ) -> Result<Self, ImageDataError> {
        let last_tile_x = top_left_corner.tile_x + ((top_left_corner.x + width) / 1000);
        let last_tile_y = top_left_corner.tile_y + ((top_left_corner.y + height) / 1000);

        let global_x_offset =
            (top_left_corner.tile_x as usize) * 1000 + (top_left_corner.x as usize);
        let global_y_offset =
            (top_left_corner.tile_y as usize) * 1000 + (top_left_corner.y as usize);

        let mut new_image = image::ImageBuffer::new(width as u32, height as u32);

        for (tile_x, tile_y) in itertools::iproduct!(
            top_left_corner.tile_x..=last_tile_x,
            top_left_corner.tile_y..=last_tile_y
        ) {
            let tile =
                TileDownloader::download(tile_x, tile_y).map_err(ImageDataError::ImageError)?;

            let initial_x_in_tile = match top_left_corner.tile_x == tile_x {
                true => top_left_corner.x,
                false => 0,
            };

            let initial_y_in_tile = match top_left_corner.tile_y == tile_y {
                true => top_left_corner.y,
                false => 0,
            };

            let final_x_in_tile = match last_tile_x == tile_x {
                true => top_left_corner.x + width - 1000 * (last_tile_x - top_left_corner.tile_x),
                false => 1000,
            };

            let final_y_in_tile = match last_tile_y == tile_y {
                true => top_left_corner.y + height - 1000 * (last_tile_y - top_left_corner.tile_y),
                false => 1000,
            };

            for (x_in_tile, y_in_tile) in itertools::iproduct!(
                initial_x_in_tile..final_x_in_tile,
                initial_y_in_tile..final_y_in_tile
            ) {
                let pixel = unsafe { tile.unsafe_get_pixel(x_in_tile as u32, y_in_tile as u32) };
                let x_global_offset = (tile_x as usize) * 1000 + (x_in_tile as usize);
                let y_global_offset = (tile_y as usize) * 1000 + (y_in_tile as usize);
                let x_in_image = x_global_offset - global_x_offset;
                let y_in_image = y_global_offset - global_y_offset;
                new_image.put_pixel(x_in_image as u32, y_in_image as u32, pixel);
            }
        }

        Self::new(new_image)
    }

    pub fn get_image(&self) -> &image::DynamicImage {
        &self.image
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_color_counts(&self) -> &HashMap<Color, u32> {
        &self.color_counts
    }

    pub fn get_total_px(&self) -> u32 {
        self.color_counts.values().sum()
    }

    pub fn get_total_time_hours(&self) -> f64 {
        convert_px_to_hours(self.get_total_px())
    }

    pub fn get_colors(&self) -> Rc<[Color]> {
        self.get_color_counts().keys().copied().collect()
    }
}
