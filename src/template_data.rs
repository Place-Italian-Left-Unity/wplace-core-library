use crate::{image_data::ImageData, tile_coords::TileCoords};

pub struct TemplateData {
    top_left_corner: TileCoords,
    image: ImageData,
}

impl TemplateData {
    pub fn new(top_left_corner: TileCoords, image: ImageData) -> Self {
        Self {
            top_left_corner,
            image,
        }
    }

    pub fn get_top_left_corner(&self) -> &TileCoords {
        &self.top_left_corner
    }

    pub fn get_image(&self) -> &ImageData {
        &self.image
    }
}
