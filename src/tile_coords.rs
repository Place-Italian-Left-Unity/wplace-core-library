#[derive(Debug)]
pub struct TileCoords {
    pub(crate) tile_x: u16,
    pub(crate) tile_y: u16,
    pub(crate) x: u16,
    pub(crate) y: u16,
}

#[derive(thiserror::Error, Debug)]
pub enum TileCoordsError {
    #[error("Parse Error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}

impl TileCoords {
    #[inline(always)]
    pub fn new(tile_x: u16, tile_y: u16, x: u16, y: u16) -> Self {
        TileCoords {
            tile_x,
            tile_y,
            x,
            y,
        }
    }

    pub fn parse_tile_coords_string(v: &str) -> Result<Self, TileCoordsError> {
        let mut buff = String::with_capacity(4);
        
        let mut out = Self::new(0, 0, 0, 0);
        
        let mut is_writing = false;
        let mut last_character = ' ';
        let mut counter = 0;
        for character in v.chars() {
            if last_character == ':' && character == ' ' {
                is_writing = true;
                counter += 1;
                continue;
            } else if character == ',' || character == ')' {
                match counter {
                    1 => out.tile_x = buff.parse()?,
                    2 => out.tile_y = buff.parse()?,
                    3 => out.x = buff.parse()?,
                    4 => { out.y = buff.parse()?; break; },
                    _ => unreachable!()
                }
                is_writing = false;
                buff.clear();
                continue;
            };
            
            if is_writing {
                match counter {
                    1..=4 => buff.push(character),
                    _ => unreachable!()
                }
            }
            
            last_character = character;
        }
        
        Ok(out)
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
