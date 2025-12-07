use crate::{
    image_data::{ImageData, ImageDataError},
    map_coords::MapCoords,
    nominatim_data::{NominatimData, NominatimDataError},
    tile_coords::{TileCoords, TileCoordsError},
};

pub enum LocationData {
    Name(String),
    Nominatim(NominatimData),
}

impl LocationData {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Name(v) => &v,
            Self::Nominatim(v) => &v.display_name,
        }
    }
}

pub struct TemplateData {
    name: String,
    top_left_corner: TileCoords,
    center_coordinates: MapCoords,
    location_data: LocationData,
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
        location_name: Option<String>,
    ) -> Result<Self, TemplateDataError> {
        let top_left_corner = TileCoords::parse_tile_coords_string(top_left_corner_coords_str)?;
        let file_path = file_path.as_ref();
        let file_name = file_path
            .file_name()
            .ok_or(TemplateDataError::NoFileName)?
            .to_str()
            .ok_or(TemplateDataError::NoFileName)?;
        let image = ImageData::new(std::fs::read(file_path)?.as_slice())?;
        Self::new(name, top_left_corner, file_name, image, location_name)
    }

    pub fn new(
        name: impl ToString,
        top_left_corner: TileCoords,
        file_name: impl ToString,
        image: ImageData,
        location_name: Option<String>,
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
            location_data: match location_name {
                Some(name) => LocationData::Name(name),
                None => match NominatimData::load_data(&center_coordinates) {
                    Ok(v) => LocationData::Nominatim(v),
                    Err(NominatimDataError::JSONDeserializeError {
                        error: _,
                        input_value: s,
                    }) if s.contains("Unable to geocode") => {
                        LocationData::Nominatim(NominatimData {
                            display_name: String::from("Unknown"),
                        })
                    }
                    Err(e) => return Err(TemplateDataError::NominatimDataError(e)),
                },
            },
            center_coordinates,
            image,
        })
    }

    pub fn download_template_area_on_map(&self) -> Result<ImageData, TemplateDataError> {
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

    pub fn get_location_data(&self) -> &LocationData {
        &self.location_data
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
