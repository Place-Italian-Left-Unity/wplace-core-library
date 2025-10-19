use std::fmt::Display;

pub struct TileCoords {
    pub(crate) tile_x: u16,
    pub(crate) tile_y: u16,
    pub(crate) x: u16,
    pub(crate) y: u16,
}

#[derive(Debug)]
pub enum TileCoordsError {
    StripSuffix,
    StripPrefix,
    DeformedTileCoords,
    ParseError(std::num::ParseIntError),
    NoTileXCoords,
    NoTileYCoords,
    NoXCoords,
    NoYCoords,
}

impl Display for TileCoordsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StripSuffix => write!(f, "Strip Suffix"),
            Self::StripPrefix => write!(f, "Strip Prefix"),
            Self::DeformedTileCoords => write!(f, "Deformed TileCoords"),
            Self::ParseError(e) => write!(f, "Parse Error: {e}"),
            Self::NoTileXCoords => write!(f, "No Tile X Coords"),
            Self::NoTileYCoords => write!(f, "No Tile Y Coords"),
            Self::NoXCoords => write!(f, "No X Coords"),
            Self::NoYCoords => write!(f, "No Y Coords"),
        }
    }
}

impl std::error::Error for TileCoordsError {}

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
