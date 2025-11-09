pub struct TileCoords {
    pub(crate) tile_x: u16,
    pub(crate) tile_y: u16,
    pub(crate) x: u16,
    pub(crate) y: u16,
}

#[derive(thiserror::Error, Debug)]
pub enum TileCoordsError {
    #[error("Strip Suffix")]
    StripSuffix,
    #[error("Strip Prefix")]
    StripPrefix,
    #[error("Deformed TileCoords")]
    DeformedTileCoords,
    #[error("Parse Error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("No Tile X Coords")]
    NoTileXCoords,
    #[error("No Tile Y Coords")]
    NoTileYCoords,
    #[error("No X Coords")]
    NoXCoords,
    #[error("No Y Coords")]
    NoYCoords,
}

impl TileCoords {
    pub fn new(tile_x: u16, tile_y: u16, x: u16, y: u16) -> Self {
        TileCoords {
            tile_x,
            tile_y,
            x,
            y,
        }
    }

    pub fn parse_tile_coords_string(v: &str) -> Result<Self, TileCoordsError> {
        let mut tile_coords = v
            .trim()
            .strip_prefix('(')
            .ok_or(TileCoordsError::StripPrefix)?
            .strip_suffix(')')
            .ok_or(TileCoordsError::StripSuffix)?
            .split(',')
            .map(|x| {
                x.split(':')
                    .next_back()
                    .ok_or(TileCoordsError::DeformedTileCoords)?
                    .trim()
                    .parse::<u16>()
                    .map_err(TileCoordsError::ParseError)
            });

        Ok(Self {
            tile_x: tile_coords.next().ok_or(TileCoordsError::NoTileXCoords)??,
            tile_y: tile_coords.next().ok_or(TileCoordsError::NoTileYCoords)??,
            x: tile_coords.next().ok_or(TileCoordsError::NoXCoords)??,
            y: tile_coords.next().ok_or(TileCoordsError::NoYCoords)??,
        })
    }

    pub fn get_x(&self) -> u16 {
        self.x
    }
    pub fn get_y(&self) -> u16 {
        self.y
    }
    pub fn get_tile_x(&self) -> u16 {
        self.tile_x
    }
    pub fn get_tile_y(&self) -> u16 {
        self.tile_y
    }
}
