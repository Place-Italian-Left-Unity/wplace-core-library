use std::{collections::HashMap, fmt::Display, rc::Rc};

use image::{GenericImage, GenericImageView};

use crate::{color::Color, image_data::ImageData};

pub struct ImageComparison {
    difference_image: image::RgbaImage,
    different_px: Rc<[(u32, u32)]>,
    difference_color_count: HashMap<Color, u32>,
}

#[derive(Debug)]
pub enum ImageComparisonError {
    IncongruentWidth,
    IncongruentHeight,
}

impl Display for ImageComparisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IncongruentHeight => "Incongruent Heights",
                Self::IncongruentWidth => "Incongruent Widths",
            }
        )
    }
}

impl std::error::Error for ImageComparisonError {}

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

        // This map will also become the missing colors output
        let mut template_colors = template_image.get_color_counts().clone();
        let current_colors = current_image.get_color_counts();

        for color in template_image.get_colors().iter() {
            let count_in_current = match current_colors.get(color) {
                Some(v) => *v,
                None => 0,
            };

            let count_in_template = unsafe { template_colors.get_mut(color).unwrap_unchecked() };

            *count_in_template -= count_in_current;
            if *count_in_template == 0 {
                unsafe {
                    template_colors.remove(color).unwrap_unchecked();
                }
            }
        }

        let difference_color_count = template_colors;

        let mut difference_image =
            image::ImageBuffer::new(template_image.width, template_image.height)
                .expand_palette(&[(0, 0, 0), (255, 0, 255)], Some(0));

        let mut different_px = Vec::new();

        for y in 0..template_image.height {
            for x in 0..template_image.width {
                if unsafe { template_image.image.unsafe_get_pixel(x, y).0 }
                    != unsafe { current_image.image.unsafe_get_pixel(x, y).0 }
                {
                    different_px.push((x, y));
                    unsafe {
                        difference_image.unsafe_put_pixel(
                            x,
                            y,
                            image::Rgba::from([255, 0, 255, 255]),
                        );
                    }
                }
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
}
