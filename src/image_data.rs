use std::{
    collections::HashMap,
    io::{BufRead, Seek},
    sync::Arc,
};

use image::{GenericImageView, ImageReader};

use crate::color::Color;

/// Currently only supports PNG
pub struct ImageData {
    image: image::DynamicImage,
    width: u32,
    height: u32,
    color_counts: HashMap<Color, u32>,
}

pub enum ImageDataError {
    ImageError(image::error::ImageError),
    InvalidWidth,
    InvalidHeight,
    InvalidColor { x: u32, y: u32, rgba: [u8; 4] },
}

impl ImageData {
    pub fn new<R: BufRead + Seek>(image_bytes: R) -> Result<Self, ImageDataError> {
        let mut image_reader = ImageReader::new(image_bytes);
        image_reader.set_format(image::ImageFormat::Png);
        let image = image_reader.decode().map_err(ImageDataError::ImageError)?;

        let width = image.width();
        if width == 0 {
            return Err(ImageDataError::InvalidWidth);
        }

        let height = image.height();
        if height == 0 {
            return Err(ImageDataError::InvalidHeight);
        }

        let mut color_counts = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let rgba = unsafe { image.unsafe_get_pixel(x, y).0 };

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
                            color_counts
                                .insert(c, 1)
                                .expect("This shouldn't be able to fail");
                        }
                    },
                }
            }
        }

        Ok(Self {
            image,
            width,
            height,
            color_counts,
        })
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
        (self.get_total_px() as f64) / 120.0
    }

    pub fn get_colors(&self) -> Arc<[Color]> {
        self.get_color_counts().keys().copied().collect()
    }
}
