use std::fmt::Display;

#[derive(Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
pub enum Color {
    Black,
    DarkGray,
    Gray,
    LightGray,
    White,
    DeepRed,
    Red,
    Orange,
    Gold,
    Yellow,
    LightYellow,
    DarkGreen,
    Green,
    LightGreen,
    DarkTeal,
    Teal,
    LightTeal,
    DarkBlue,
    Blue,
    Cyan,
    Indigo,
    LightIndigo,
    DarkPurple,
    Purple,
    LightPurple,
    DarkPink,
    Pink,
    LightPink,
    DarkBrown,
    Brown,
    Beige,
    Transparent,
    MediumGray,
    DarkRed,
    LightRed,
    DarkOrange,
    DarkGoldenrod,
    Goldenrod,
    LightGoldenrod,
    DarkOlive,
    Olive,
    LightOlive,
    DarkCyan,
    LightCyan,
    LightBlue,
    DarkIndigo,
    DarkSlateBlue,
    SlateBlue,
    LightSlateBlue,
    DarkPeach,
    Peach,
    LightPeach,
    LightBrown,
    DarkTan,
    Tan,
    LightTan,
    DarkBeige,
    LightBeige,
    DarkStone,
    Stone,
    LightStone,
    DarkSlate,
    Slate,
    LightSlate,
}

impl Color {
    pub fn is_premium(&self) -> bool {
        match self {
            Self::Black
            | Self::DarkGray
            | Self::Gray
            | Self::LightGray
            | Self::White
            | Self::DeepRed
            | Self::Red
            | Self::Orange
            | Self::Gold
            | Self::Yellow
            | Self::LightYellow
            | Self::DarkGreen
            | Self::Green
            | Self::LightGreen
            | Self::DarkTeal
            | Self::Teal
            | Self::LightTeal
            | Self::DarkBlue
            | Self::Blue
            | Self::Cyan
            | Self::Indigo
            | Self::LightIndigo
            | Self::DarkPurple
            | Self::Purple
            | Self::LightPurple
            | Self::DarkPink
            | Self::Pink
            | Self::LightPink
            | Self::DarkBrown
            | Self::Brown
            | Self::Beige
            | Self::Transparent => false,
            Self::MediumGray
            | Self::DarkRed
            | Self::LightRed
            | Self::DarkOrange
            | Self::DarkGoldenrod
            | Self::Goldenrod
            | Self::LightGoldenrod
            | Self::DarkOlive
            | Self::Olive
            | Self::LightOlive
            | Self::DarkCyan
            | Self::LightCyan
            | Self::LightBlue
            | Self::DarkIndigo
            | Self::DarkSlateBlue
            | Self::SlateBlue
            | Self::LightSlateBlue
            | Self::DarkPeach
            | Self::Peach
            | Self::LightPeach
            | Self::LightBrown
            | Self::DarkTan
            | Self::Tan
            | Self::LightTan
            | Self::DarkBeige
            | Self::LightBeige
            | Self::DarkStone
            | Self::Stone
            | Self::LightStone
            | Self::DarkSlate
            | Self::Slate
            | Self::LightSlate => true,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Black => "Black",
                Self::DarkGray => "Dark Gray",
                Self::Gray => "Gray",
                Self::LightGray => "Light Gray",
                Self::White => "White",
                Self::DeepRed => "Deep Red",
                Self::Red => "Red",
                Self::Orange => "Orange",
                Self::Gold => "Gold",
                Self::Yellow => "Yellow",
                Self::LightYellow => "Light Yellow",
                Self::DarkGreen => "Dark Green",
                Self::Green => "Green",
                Self::LightGreen => "Light Green",
                Self::DarkTeal => "Dark Teal",
                Self::Teal => "Teal",
                Self::LightTeal => "Light Teal",
                Self::DarkBlue => "Dark Blue",
                Self::Blue => "Blue",
                Self::Cyan => "Cyan",
                Self::Indigo => "Indigo",
                Self::LightIndigo => "Light Indigo",
                Self::DarkPurple => "Dark Purple",
                Self::Purple => "Purple",
                Self::LightPurple => "Light Purple",
                Self::DarkPink => "Dark Pink",
                Self::Pink => "Pink",
                Self::LightPink => "Light Pink",
                Self::DarkBrown => "Dark Brown",
                Self::Brown => "Brown",
                Self::Beige => "Beige",
                Self::Transparent => "Transparent",
                Self::MediumGray => "Medium Gray",
                Self::DarkRed => "Dark Red",
                Self::LightRed => "Light Red",
                Self::DarkOrange => "Dark Orange",
                Self::DarkGoldenrod => "Dark Goldenrod",
                Self::Goldenrod => "Goldenrod",
                Self::LightGoldenrod => "Light Goldenrod",
                Self::DarkOlive => "Dark Olive",
                Self::Olive => "Olive",
                Self::LightOlive => "Light Olive",
                Self::DarkCyan => "Dark Cyan",
                Self::LightCyan => "Light Cyan",
                Self::LightBlue => "Light Blue",
                Self::DarkIndigo => "Dark Indigo",
                Self::DarkSlateBlue => "Dark Slate Blue",
                Self::SlateBlue => "Slate Blue",
                Self::LightSlateBlue => "Light Slate Blue",
                Self::DarkPeach => "Dark Peach",
                Self::Peach => "Peach",
                Self::LightPeach => "Light Peach",
                Self::LightBrown => "Light Brown",
                Self::DarkTan => "Dark Tan",
                Self::Tan => "Tan",
                Self::LightTan => "Light Tan",
                Self::DarkBeige => "Dark Beige",
                Self::LightBeige => "Light Beige",
                Self::DarkStone => "Dark Stone",
                Self::Stone => "Stone",
                Self::LightStone => "Light Stone",
                Self::DarkSlate => "Dark Slate",
                Self::Slate => "Slate",
                Self::LightSlate => "Light Slate",
            }
        )
    }
}

impl TryFrom<[u8; 4]> for Color {
    type Error = ();
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match value {
            [_, _, _, alpha] if alpha != u8::MAX => Err(()),
            [0x00, 0x00, 0x00, _] => Ok(Self::Black),
            [0x3C, 0x3C, 0x3C, _] => Ok(Self::DarkGray),
            [0x78, 0x78, 0x78, _] => Ok(Self::Gray),
            [0xD2, 0xD2, 0xD2, _] => Ok(Self::LightGray),
            [0xFF, 0xFF, 0xFF, _] => Ok(Self::White),
            [0x60, 0x00, 0x18, _] => Ok(Self::DeepRed),
            [0xED, 0x1C, 0x24, _] => Ok(Self::Red),
            [0xFF, 0x7F, 0x27, _] => Ok(Self::Orange),
            [0xF6, 0xAA, 0x09, _] => Ok(Self::Gold),
            [0xF9, 0xDD, 0x3B, _] => Ok(Self::Yellow),
            [0xFF, 0xFA, 0xBC, _] => Ok(Self::LightYellow),
            [0x0E, 0xB9, 0x68, _] => Ok(Self::DarkGreen),
            [0x13, 0xE6, 0x7B, _] => Ok(Self::Green),
            [0x87, 0xFF, 0x5E, _] => Ok(Self::LightGreen),
            [0x0C, 0x81, 0x6E, _] => Ok(Self::DarkTeal),
            [0x10, 0xAE, 0xA6, _] => Ok(Self::Teal),
            [0x13, 0xE1, 0xBE, _] => Ok(Self::LightTeal),
            [0x60, 0xF7, 0xF2, _] => Ok(Self::DarkBlue),
            [0x28, 0x50, 0x9E, _] => Ok(Self::Blue),
            [0x40, 0x93, 0xE4, _] => Ok(Self::Cyan),
            [0x6B, 0x50, 0xF6, _] => Ok(Self::Indigo),
            [0x99, 0xB1, 0xFB, _] => Ok(Self::LightIndigo),
            [0x78, 0x0C, 0x99, _] => Ok(Self::DarkPurple),
            [0xAA, 0x38, 0xB9, _] => Ok(Self::Purple),
            [0xE0, 0x9F, 0xF9, _] => Ok(Self::LightPurple),
            [0xCB, 0x00, 0x7A, _] => Ok(Self::DarkPink),
            [0xEC, 0x1F, 0x80, _] => Ok(Self::Pink),
            [0xF3, 0x8D, 0xA9, _] => Ok(Self::LightPink),
            [0x68, 0x46, 0x34, _] => Ok(Self::DarkBrown),
            [0x95, 0x68, 0x2A, _] => Ok(Self::Brown),
            [0xF8, 0xB2, 0x77, _] => Ok(Self::Beige),
            [0xDE, 0xFA, 0xCE, _] => Ok(Self::Transparent),
            [0xAA, 0xAA, 0xAA, _] => Ok(Self::MediumGray),
            [0xA5, 0x0E, 0x1E, _] => Ok(Self::DarkRed),
            [0xFA, 0x80, 0x72, _] => Ok(Self::LightRed),
            [0xE4, 0x5C, 0x1A, _] => Ok(Self::DarkOrange),
            [0x9C, 0x84, 0x31, _] => Ok(Self::DarkGoldenrod),
            [0xC5, 0xAD, 0x31, _] => Ok(Self::Goldenrod),
            [0xE8, 0xD4, 0x5F, _] => Ok(Self::LightGoldenrod),
            [0x4A, 0x6B, 0x3A, _] => Ok(Self::DarkOlive),
            [0x5A, 0x94, 0x4A, _] => Ok(Self::Olive),
            [0x84, 0xC5, 0x73, _] => Ok(Self::LightOlive),
            [0x0F, 0x79, 0x9F, _] => Ok(Self::DarkCyan),
            [0xBB, 0xFA, 0xF2, _] => Ok(Self::LightCyan),
            [0x7D, 0xC7, 0xFF, _] => Ok(Self::LightBlue),
            [0x4D, 0x31, 0xB8, _] => Ok(Self::DarkIndigo),
            [0x4A, 0x42, 0x84, _] => Ok(Self::DarkSlateBlue),
            [0x7A, 0x71, 0xC4, _] => Ok(Self::SlateBlue),
            [0xB5, 0xAE, 0xF1, _] => Ok(Self::LightSlateBlue),
            [0x9B, 0x52, 0x49, _] => Ok(Self::DarkPeach),
            [0xD1, 0x80, 0x78, _] => Ok(Self::Peach),
            [0xFA, 0xB6, 0xA4, _] => Ok(Self::LightPeach),
            [0xDB, 0xA4, 0x63, _] => Ok(Self::LightBrown),
            [0x7B, 0x63, 0x52, _] => Ok(Self::DarkTan),
            [0x9C, 0x84, 0x6B, _] => Ok(Self::Tan),
            [0xD6, 0xB5, 0x94, _] => Ok(Self::LightTan),
            [0xD1, 0x80, 0x51, _] => Ok(Self::DarkBeige),
            [0xFF, 0xC5, 0xA5, _] => Ok(Self::LightBeige),
            [0x6D, 0x64, 0x3F, _] => Ok(Self::DarkStone),
            [0x94, 0x8C, 0x6B, _] => Ok(Self::Stone),
            [0xCD, 0xC5, 0x9E, _] => Ok(Self::LightStone),
            [0x33, 0x39, 0x41, _] => Ok(Self::DarkSlate),
            [0x6D, 0x75, 0x8D, _] => Ok(Self::Slate),
            [0xB3, 0xB9, 0xD1, _] => Ok(Self::LightSlate),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::DarkGray),
            2 => Ok(Self::Gray),
            3 => Ok(Self::LightGray),
            4 => Ok(Self::White),
            5 => Ok(Self::DeepRed),
            6 => Ok(Self::Red),
            7 => Ok(Self::Orange),
            8 => Ok(Self::Gold),
            9 => Ok(Self::Yellow),
            10 => Ok(Self::LightYellow),
            11 => Ok(Self::DarkGreen),
            12 => Ok(Self::Green),
            13 => Ok(Self::LightGreen),
            14 => Ok(Self::DarkTeal),
            15 => Ok(Self::Teal),
            16 => Ok(Self::LightTeal),
            17 => Ok(Self::DarkBlue),
            18 => Ok(Self::Blue),
            19 => Ok(Self::Cyan),
            20 => Ok(Self::Indigo),
            21 => Ok(Self::LightIndigo),
            22 => Ok(Self::DarkPurple),
            23 => Ok(Self::Purple),
            24 => Ok(Self::LightPurple),
            25 => Ok(Self::DarkPink),
            26 => Ok(Self::Pink),
            27 => Ok(Self::LightPink),
            28 => Ok(Self::DarkBrown),
            29 => Ok(Self::Brown),
            30 => Ok(Self::Beige),
            31 => Ok(Self::Transparent),
            32 => Ok(Self::MediumGray),
            33 => Ok(Self::DarkRed),
            34 => Ok(Self::LightRed),
            35 => Ok(Self::DarkOrange),
            36 => Ok(Self::DarkGoldenrod),
            37 => Ok(Self::Goldenrod),
            38 => Ok(Self::LightGoldenrod),
            39 => Ok(Self::DarkOlive),
            40 => Ok(Self::Olive),
            41 => Ok(Self::LightOlive),
            42 => Ok(Self::DarkCyan),
            43 => Ok(Self::LightCyan),
            44 => Ok(Self::LightBlue),
            45 => Ok(Self::DarkIndigo),
            46 => Ok(Self::DarkSlateBlue),
            47 => Ok(Self::SlateBlue),
            48 => Ok(Self::LightSlateBlue),
            49 => Ok(Self::DarkPeach),
            50 => Ok(Self::Peach),
            51 => Ok(Self::LightPeach),
            52 => Ok(Self::LightBrown),
            53 => Ok(Self::DarkTan),
            54 => Ok(Self::Tan),
            55 => Ok(Self::LightTan),
            56 => Ok(Self::DarkBeige),
            57 => Ok(Self::LightBeige),
            58 => Ok(Self::DarkStone),
            59 => Ok(Self::Stone),
            60 => Ok(Self::LightStone),
            61 => Ok(Self::DarkSlate),
            62 => Ok(Self::Slate),
            63 => Ok(Self::LightSlate),
            _ => Err(()),
        }
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => [0x00, 0x00, 0x00, 255],
            Color::DarkGray => [0x3C, 0x3C, 0x3C, 255],
            Color::Gray => [0x78, 0x78, 0x78, 255],
            Color::LightGray => [0xD2, 0xD2, 0xD2, 255],
            Color::White => [0xFF, 0xFF, 0xFF, 255],
            Color::DeepRed => [0x60, 0x00, 0x18, 255],
            Color::Red => [0xED, 0x1C, 0x24, 255],
            Color::Orange => [0xFF, 0x7F, 0x27, 255],
            Color::Gold => [0xF6, 0xAA, 0x09, 255],
            Color::Yellow => [0xF9, 0xDD, 0x3B, 255],
            Color::LightYellow => [0xFF, 0xFA, 0xBC, 255],
            Color::DarkGreen => [0x0E, 0xB9, 0x68, 255],
            Color::Green => [0x13, 0xE6, 0x7B, 255],
            Color::LightGreen => [0x87, 0xFF, 0x5E, 255],
            Color::DarkTeal => [0x0C, 0x81, 0x6E, 255],
            Color::Teal => [0x10, 0xAE, 0xA6, 255],
            Color::LightTeal => [0x13, 0xE1, 0xBE, 255],
            Color::DarkBlue => [0x60, 0xF7, 0xF2, 255],
            Color::Blue => [0x28, 0x50, 0x9E, 255],
            Color::Cyan => [0x40, 0x93, 0xE4, 255],
            Color::Indigo => [0x6B, 0x50, 0xF6, 255],
            Color::LightIndigo => [0x99, 0xB1, 0xFB, 255],
            Color::DarkPurple => [0x78, 0x0C, 0x99, 255],
            Color::Purple => [0xAA, 0x38, 0xB9, 255],
            Color::LightPurple => [0xE0, 0x9F, 0xF9, 255],
            Color::DarkPink => [0xCB, 0x00, 0x7A, 255],
            Color::Pink => [0xEC, 0x1F, 0x80, 255],
            Color::LightPink => [0xF3, 0x8D, 0xA9, 255],
            Color::DarkBrown => [0x68, 0x46, 0x34, 255],
            Color::Brown => [0x95, 0x68, 0x2A, 255],
            Color::Beige => [0xF8, 0xB2, 0x77, 255],
            Color::Transparent => [0xDE, 0xFA, 0xCE, 255],
            Color::MediumGray => [0xAA, 0xAA, 0xAA, 255],
            Color::DarkRed => [0xA5, 0x0E, 0x1E, 255],
            Color::LightRed => [0xFA, 0x80, 0x72, 255],
            Color::DarkOrange => [0xE4, 0x5C, 0x1A, 255],
            Color::DarkGoldenrod => [0x9C, 0x84, 0x31, 255],
            Color::Goldenrod => [0xC5, 0xAD, 0x31, 255],
            Color::LightGoldenrod => [0xE8, 0xD4, 0x5F, 255],
            Color::DarkOlive => [0x4A, 0x6B, 0x3A, 255],
            Color::Olive => [0x5A, 0x94, 0x4A, 255],
            Color::LightOlive => [0x84, 0xC5, 0x73, 255],
            Color::DarkCyan => [0x0F, 0x79, 0x9F, 255],
            Color::LightCyan => [0xBB, 0xFA, 0xF2, 255],
            Color::LightBlue => [0x7D, 0xC7, 0xFF, 255],
            Color::DarkIndigo => [0x4D, 0x31, 0xB8, 255],
            Color::DarkSlateBlue => [0x4A, 0x42, 0x84, 255],
            Color::SlateBlue => [0x7A, 0x71, 0xC4, 255],
            Color::LightSlateBlue => [0xB5, 0xAE, 0xF1, 255],
            Color::DarkPeach => [0x9B, 0x52, 0x49, 255],
            Color::Peach => [0xD1, 0x80, 0x78, 255],
            Color::LightPeach => [0xFA, 0xB6, 0xA4, 255],
            Color::LightBrown => [0xDB, 0xA4, 0x63, 255],
            Color::DarkTan => [0x7B, 0x63, 0x52, 255],
            Color::Tan => [0x9C, 0x84, 0x6B, 255],
            Color::LightTan => [0xD6, 0xB5, 0x94, 255],
            Color::DarkBeige => [0xD1, 0x80, 0x51, 255],
            Color::LightBeige => [0xFF, 0xC5, 0xA5, 255],
            Color::DarkStone => [0x6D, 0x64, 0x3F, 255],
            Color::Stone => [0x94, 0x8C, 0x6B, 255],
            Color::LightStone => [0xCD, 0xC5, 0x9E, 255],
            Color::DarkSlate => [0x33, 0x39, 0x41, 255],
            Color::Slate => [0x6D, 0x75, 0x8D, 255],
            Color::LightSlate => [0xB3, 0xB9, 0xD1, 255],
        }
    }
}
