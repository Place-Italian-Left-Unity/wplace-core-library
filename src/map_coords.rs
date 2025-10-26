use crate::tile_coords::TileCoords;

pub struct MapCoords {
    lat: f64,
    lng: f64,
    zoom: f32,
}

impl MapCoords {
    pub fn get_lat(&self) -> f64 {
        self.lat
    }
    pub fn get_lng(&self) -> f64 {
        self.lng
    }
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    pub fn get_link(&self) -> String {
        format!(
            "https://wplace.live/?lat={}&lng={}&zoom={}",
            self.lat,
            self.lng,
            self.get_zoom()
        )
    }
    
    pub fn get_nominatim_link(&self) -> String {
        format!("https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&zoom=13&format=jsonv2", self.lat, self.lng)
    }

    pub fn from_tile_coords(tile_coords: &TileCoords, width: u32, height: u32) -> Self {
        let rel_x = ((tile_coords.get_tile_x() as f64) * 1000f64 + (tile_coords.get_x() as f64))
            / (2048f64 * 1000f64); // Relative X
        let rel_y = 1f64
            - ((tile_coords.get_tile_y() as f64) * 1000f64 + (tile_coords.get_y() as f64))
                / (2048f64 * 1000f64); // Relative Y
        Self {
            lat: 360f64
                * (std::f64::consts::E.powf((rel_y * 2f64 - 1f64) * std::f64::consts::PI)).atan()
                / std::f64::consts::PI
                - 90f64,
            lng: rel_x * 360f64 - 180f64,
            zoom: match std::cmp::max(width, height) {
                0..1 => 22.0,
                1..10 => 20.0,
                10..60 => 17.0,
                60..100 => 14.0,
                100..300 => 13.0,
                300..500 => 12.5,
                500..1000 => 12.0,
                _ => 11.0,
            },
        }
    }
}
