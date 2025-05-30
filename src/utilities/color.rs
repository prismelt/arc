#[derive(Debug, PartialEq)]
pub enum ColorLiteral {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
    Black,
    White,
    Gray,
    Brown,
    Pink,
    Purple,
    Cyan,
    Magenta,
    Lime,
    Teal,
    Maroon,
    Navy,
}
#[derive(Debug, PartialEq)]
pub enum Color {
    Literal(ColorLiteral),
    RGB(u8, u8, u8),
}

impl Color {
    pub fn build(&self) -> String {
        match self {
            Color::RGB(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            Color::Literal(_) => {
                let (r, g, b) = self.to_rgb();
                format!("rgb({}, {}, {})", r, g, b)
            }
        }
    }

    pub fn from_string(string: String) -> Result<Self, String> {
        if string.starts_with("(") {
            let mut rgb: std::str::Split<'_, &str> = string[1..string.len() - 1].split(",");
            if rgb.clone().count() == 3 {
                let r = Self::parse_rgb_literal(&mut rgb)?;
                let g = Self::parse_rgb_literal(&mut rgb)?;
                let b = Self::parse_rgb_literal(&mut rgb)?;
                return Ok(Color::new_rgb(r, g, b));
            }

            if rgb.clone().count() > 3 {
                return Err(format!("Too many values for rgb literal: {}", string));
            } else {
                return Err(format!("Insufficient values for rgb literal: {}", string));
            }
        } else {
            Ok(Color::new_literal(Self::parse_str_literal(&string)?))
        }
    }

    fn parse_str_literal(string: &String) -> Result<ColorLiteral, String> {
        match string.to_lowercase().trim() {
            "red" => Ok(ColorLiteral::Red),
            "orange" => Ok(ColorLiteral::Orange),
            "yellow" => Ok(ColorLiteral::Yellow),
            "green" => Ok(ColorLiteral::Green),
            "blue" => Ok(ColorLiteral::Blue),
            "indigo" => Ok(ColorLiteral::Indigo),
            "violet" => Ok(ColorLiteral::Violet),
            "black" => Ok(ColorLiteral::Black),
            "white" => Ok(ColorLiteral::White),
            "gray" => Ok(ColorLiteral::Gray),
            "brown" => Ok(ColorLiteral::Brown),
            "pink" => Ok(ColorLiteral::Pink),
            "purple" => Ok(ColorLiteral::Purple),
            "cyan" => Ok(ColorLiteral::Cyan),
            "magenta" => Ok(ColorLiteral::Magenta),
            "lime" => Ok(ColorLiteral::Lime),
            "teal" => Ok(ColorLiteral::Teal),
            "maroon" => Ok(ColorLiteral::Maroon),
            "navy" => Ok(ColorLiteral::Navy),
            _ => Err(format!("Invalid color literal: {}", string)),
        }
    }

    fn parse_rgb_literal(rgb: &mut std::str::Split<'_, &str>) -> Result<u8, String> {
        let value = rgb.next().unwrap();
        let parsed_value = value.trim().parse::<u8>();
        if parsed_value.is_err() {
            return Err(format!("Invalid value for rgb literal: {}", value));
        }
        Ok(parsed_value.unwrap())
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Literal(literal) => match literal {
                ColorLiteral::Red => (255, 0, 0),
                ColorLiteral::Orange => (255, 165, 0),
                ColorLiteral::Yellow => (255, 255, 0),
                ColorLiteral::Green => (0, 255, 0),
                ColorLiteral::Blue => (0, 0, 255),
                ColorLiteral::Indigo => (75, 0, 130),
                ColorLiteral::Violet => (127, 0, 255),
                ColorLiteral::Black => (0, 0, 0),
                ColorLiteral::White => (255, 255, 255),
                ColorLiteral::Gray => (128, 128, 128),
                ColorLiteral::Brown => (165, 42, 42),
                ColorLiteral::Pink => (255, 192, 203),
                ColorLiteral::Purple => (128, 0, 128),
                ColorLiteral::Cyan => (0, 255, 255),
                ColorLiteral::Magenta => (255, 0, 255),
                ColorLiteral::Lime => (0, 255, 0),
                ColorLiteral::Teal => (0, 128, 128),
                ColorLiteral::Maroon => (128, 0, 0),
                ColorLiteral::Navy => (0, 0, 128),
            },
            Color::RGB(r, g, b) => (*r, *g, *b),
        }
    }

    fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::RGB(r, g, b)
    }
    fn new_literal(literal: ColorLiteral) -> Self {
        Self::Literal(literal)
    }
}
