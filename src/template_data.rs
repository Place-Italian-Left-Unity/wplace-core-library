use crate::{
    image_data::{ImageData, ImageDataError},
    map_coords::MapCoords,
    nominatim_data::{NominatimData, NominatimDataError},
    tile_coords::{TileCoords, TileCoordsError},
};

pub struct TemplateData {
    name: String,
    top_left_corner: TileCoords,
    center_coordinates: MapCoords,
    nominatim_data: NominatimData,
    image: ImageData,
    file_name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum TemplateDataError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("ImageData Error: {0}")]
    ImageDataError(#[from] ImageDataError),
    #[error("TileCoords Error: {0}")]
    TileCoordsError(#[from] TileCoordsError),
    #[error("NominatimData Error: {0}")]
    NominatimDataError(#[from] NominatimDataError),
    #[error("No file name")]
    NoFileName,
}

impl TemplateData {
    pub fn from_data<P: AsRef<std::path::Path>>(
        name: impl ToString,
        top_left_corner_coords_str: &str,
        file_path: P,
    ) -> Result<Self, TemplateDataError> {
        let top_left_corner = TileCoords::parse_tile_coords_string(top_left_corner_coords_str)?;
        let file_path = file_path.as_ref();
        let file_name = file_path
            .file_name()
            .ok_or(TemplateDataError::NoFileName)?
            .to_str()
            .ok_or(TemplateDataError::NoFileName)?;
        let image = ImageData::new(std::fs::read(file_path)?.as_slice())?;
        Self::new(name, top_left_corner, file_name, image)
    }

    pub fn new(
        name: impl ToString,
        top_left_corner: TileCoords,
        file_name: impl ToString,
        image: ImageData,
    ) -> Result<Self, TemplateDataError> {
        let center_coordinates = MapCoords::from_tile_coords(
            &TileCoords {
                tile_x: top_left_corner.tile_x,
                tile_y: top_left_corner.tile_y,
                x: top_left_corner.x + (image.width as u16),
                y: top_left_corner.y + (image.height as u16),
            },
            image.width,
            image.height,
        );
        Ok(Self {
            name: name.to_string(),
            file_name: file_name.to_string(),
            top_left_corner,
            nominatim_data: match NominatimData::load_data(&center_coordinates) {
                Ok(v) => v,
                Err(NominatimDataError::JSONDeserializeError {
                    error: _,
                    input_value: s,
                }) if s.contains("Unable to geocode") => NominatimData {
                    display_name: String::from("Unknown"),
                },
                Err(e) => return Err(TemplateDataError::NominatimDataError(e)),
            },
            center_coordinates,
            image,
        })
    }

    pub fn get_template_area(&self) -> Result<ImageData, TemplateDataError> {
        ImageData::from_site_coords(
            &self.top_left_corner,
            self.image.width as u16,
            self.image.height as u16,
        )
        .map_err(From::from)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_nominatim_data(&self) -> &NominatimData {
        &self.nominatim_data
    }

    pub fn get_top_left_corner(&self) -> &TileCoords {
        &self.top_left_corner
    }

    pub fn get_image(&self) -> &ImageData {
        &self.image
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn get_center_coordiantes(&self) -> &MapCoords {
        &self.center_coordinates
    }
}
