pub struct ImageInfo {
    file_name: String,
    image: image::DynamicImage,
    width: u32,
    height: u32,
}

impl ImageInfo {
    pub fn get_file_name(&self) -> &str {
        &self.file_name
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
}
