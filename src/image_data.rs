use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, Seek},
    rc::Rc,
};

use image::{GenericImageView, ImageReader};

use crate::{color::Color, convert_px_to_hours};

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
            Self::InvalidColor {
                x,
                y,
                rgba: [r, g, b, a],
            } => write!(f, "Invalid color [{r},{g},{b},{a}] at {x}, {y}"),
        }
    }
}
impl std::error::Error for ImageDataError {}

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

        for pixel in image.pixels() {
            let (x, y, pixel) = pixel;
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
                    None => unsafe {
                        // This can never possibly fail
                        color_counts.insert(c, 1).unwrap_unchecked();
                    },
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
