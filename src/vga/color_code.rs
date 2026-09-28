#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Color {
    pub fn from_name(s: &str) -> Color {
        match s {
            "black" => Color::Black,
            "blue" => Color::Blue,
            "green" => Color::Green,
            "cyan" => Color::Cyan,
            "red" => Color::Red,
            "magenta" => Color::Magenta,
            "brown" => Color::Brown,
            "lightgray" => Color::LightGray,
            "darkgray" => Color::DarkGray,
            "lightgreen" => Color::LightGreen,
            "lightcyan" => Color::LightCyan,
            "lightred" => Color::LightRed,
            "pink" => Color::Pink,
            "yellow" => Color::Yellow,
            "white" => Color::White,
            _ => panic!("{} is not a valid color!", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> Self {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }

    pub const RED: ColorCode = Self((Color::Black as u8) << 4 | (Color::Red as u8));
    pub const BLUE: ColorCode = Self((Color::Black as u8) << 4 | (Color::Blue as u8));
    pub const YELLOW: ColorCode = Self((Color::Black as u8) << 4 | (Color::Yellow as u8));
    pub const GREEN: ColorCode = Self((Color::Black as u8) << 4 | (Color::Green as u8));
}


