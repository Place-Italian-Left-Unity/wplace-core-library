use std::{collections::HashMap, rc::Rc};

use image::{GenericImage, GenericImageView};

use crate::{color::Color, convert_px_to_hours, image_data::ImageData};

pub struct ImageComparison {
    difference_image: image::RgbaImage,
    different_px: Rc<[(u32, u32)]>,
    difference_color_count: HashMap<Color, u32>,
}

#[derive(thiserror::Error, Debug)]
pub enum ImageComparisonError {
    #[error("Incongruent Heights")]
    IncongruentWidth,
    #[error("Incongruent Widths")]
    IncongruentHeight,
}

impl ImageComparison {
    /// * `template_image` - Template of the image one is looking at
    /// * `current_image` - should be what the template is being compared to, e.g. the image ripped from wplace.live
    pub fn compare_images(
        template_image: &ImageData,
        current_image: &ImageData,
    ) -> Result<Self, ImageComparisonError> {
        if template_image.height != current_image.height {
            return Err(ImageComparisonError::IncongruentHeight);
        }

        if template_image.width != current_image.width {
            return Err(ImageComparisonError::IncongruentWidth);
        }

        let mut difference_color_count = HashMap::new();

        let mut difference_image =
            image::ImageBuffer::new(template_image.width, template_image.height)
                .expand_palette(&[(0, 0, 0), (255, 0, 255)], Some(0));

        let mut different_px = Vec::new();

        for (x, y, pixel) in template_image.image.pixels() {
            if pixel.0 == unsafe { current_image.image.unsafe_get_pixel(x, y).0 } {
                continue;
            }

            // Color validity can't fail because it has been checked in the Image already
            let color = unsafe { Color::try_from(pixel.0).unwrap_unchecked() };

            match difference_color_count.get_mut(&color) {
                Some(v) => *v += 1,
                None => {
                    let _ = difference_color_count.insert(color, 1);
                }
            }

            different_px.push((x, y));
            unsafe {
                difference_image.unsafe_put_pixel(x, y, image::Rgba::from([255, 0, 255, 255]));
            }
        }

        Ok(Self {
            difference_color_count,
            difference_image,
            different_px: different_px.into(),
        })
    }

    pub fn get_difference_color_count(&self) -> &HashMap<Color, u32> {
        &self.difference_color_count
    }

    pub fn get_difference_image(&self) -> &image::RgbaImage {
        &self.difference_image
    }

    pub fn get_different_px(&self) -> Rc<[(u32, u32)]> {
        self.different_px.clone()
    }

    pub fn get_total_different_px(&self) -> u32 {
        self.difference_color_count.values().sum()
    }

    pub fn get_total_time_hours(&self) -> f64 {
        convert_px_to_hours(self.get_total_different_px())
    }
}
