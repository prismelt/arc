use crate::types::color::Color;
use maud::html;

#[derive(Debug, PartialEq)]
pub enum MetaProperties {
    Name(String),
    Title(String),
    FontFamily(String),
    FontSize(u8),
    FontColor(Color),
    BackgroundColor(Color),
    AllowHtml(bool),
    PTagFontSize(u8),
    PTagFontColor(Color),
    H1TagFontSize(u8),
    H1TagFontColor(Color),
    H2TagFontSize(u8),
    H2TagFontColor(Color),
    H3TagFontSize(u8),
    H3TagFontColor(Color),
    H4TagFontSize(u8),
    H4TagFontColor(Color),
}

impl MetaProperties {
    pub fn new(string: String) -> Option<Self> {
        let parts: Vec<&str> = string.splitn(2, "=").collect();
        if parts.len() != 2 {
            panic!("Invalid <meta /> property: {}", string);
        }
        let key = parts[0].trim();
        let value = parts[1].trim();

        if value.is_empty() {
            eprintln!("Invalid <meta /> property: {}", string);
            return None;
        }
        Self::convert_meta_result(key, value)
    }

    pub fn build(&self) -> String {
        let result = match self {
            MetaProperties::Name(_) => String::new(),
            MetaProperties::Title(title) => html! { title { (title) } }.into_string(),
            MetaProperties::FontFamily(family) => {
                html! { style { "* { font-family: " (family) "; }" } }.into_string()
            }
            MetaProperties::FontSize(size) => {
                html! { style { "span { font-size: " (size) "px; }" } }.into_string()
            }
            MetaProperties::FontColor(color) => {
                html! { style { "span { color: " (color.build()) "; }" } }.into_string()
            }
            MetaProperties::BackgroundColor(color) => {
                html! { style { "html, body, main { background-color: " (color.build()) "; }" } }
                    .into_string()
            }
            MetaProperties::AllowHtml(_) => String::new(),
            MetaProperties::PTagFontSize(size) => {
                html! { style { "p { font-size: " (size) "px !important; }" } }.into_string()
            }
            MetaProperties::PTagFontColor(color) => {
                html! { style { "p { color: " (color.build()) "; }" } }.into_string()
            }
            MetaProperties::H1TagFontSize(size) => {
                html!({ style { ".h1size { font-size: " (size) "px !important; }" } }).into_string()
            }
            MetaProperties::H1TagFontColor(color) => {
                html! { style { ".h1size { color: " (color.build()) "; }" } }.into_string()
            }
            MetaProperties::H2TagFontSize(size) => {
                html!({ style { ".h2size { font-size: " (size) "px !important; }" } }).into_string()
            }
            MetaProperties::H2TagFontColor(color) => {
                html! { style { ".h2size { color: " (color.build()) "; }" } }.into_string()
            }
            MetaProperties::H3TagFontSize(size) => {
                html!({ style { ".h3size { font-size: " (size) "px !important; }" } }).into_string()
            }
            MetaProperties::H3TagFontColor(color) => {
                html! { style { ".h3size { color: " (color.build()) "; }" } }.into_string()
            }
            MetaProperties::H4TagFontSize(size) => {
                html!({ style { ".h4size { font-size: " (size) "px !important; }" } }).into_string()
            }
            MetaProperties::H4TagFontColor(color) => {
                html! { style { ".h4size { color: " (color.build()) "; }" } }.into_string()
            }
        };
        result
    }

    fn convert_meta_result(key: &str, value: &str) -> Option<MetaProperties> {
        match key {
            "name" => Some(MetaProperties::Name(value.to_string())),
            "title" => Some(MetaProperties::Title(value.to_string())),
            "font-family" => Some(MetaProperties::FontFamily(value.to_string())),
            "font-size" => Some(MetaProperties::FontSize(Self::str_to_u8(value))),
            "font-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::FontColor(color))
                } else {
                    None
                }
            }
            "background-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::BackgroundColor(color))
                } else {
                    None
                }
            }
            "text-font-size" => Some(MetaProperties::PTagFontSize(Self::str_to_u8(value))),
            "text-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::PTagFontColor(color))
                } else {
                    None
                }
            }
            "h1-font-size" => Some(MetaProperties::H1TagFontSize(Self::str_to_u8(value))),
            "h1-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::H1TagFontColor(color))
                } else {
                    None
                }
            }
            "h2-font-size" => Some(MetaProperties::H2TagFontSize(Self::str_to_u8(value))),
            "h2-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::H2TagFontColor(color))
                } else {
                    None
                }
            }
            "h3-font-size" => Some(MetaProperties::H3TagFontSize(Self::str_to_u8(value))),
            "h3-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::H3TagFontColor(color))
                } else {
                    None
                }
            }
            "h4-font-size" => Some(MetaProperties::H4TagFontSize(Self::str_to_u8(value))),
            "h4-color" => {
                if let Some(color) = Self::str_to_color(value) {
                    Some(MetaProperties::H4TagFontColor(color))
                } else {
                    None
                }
            }
            "allow-html" => {
                let value = value.trim().to_lowercase();
                match value.as_str() {
                    "true" => Some(MetaProperties::AllowHtml(true)),
                    "false" => Some(MetaProperties::AllowHtml(false)),
                    _ => panic!("Invalid boolean value for allow-html: {}", value),
                }
            }
            _ => panic!("Invalid <meta /> property: {}", key),
        }
    }

    fn str_to_color(str: &str) -> Option<Color> {
        let result = Color::from_string(str.to_string());
        if let Err(err) = result {
            eprintln!("{}", err);
            return None;
        }
        result.ok()
    }

    fn str_to_u8(string: &str) -> u8 {
        let result = string.parse::<u8>();
        if result.is_err() {
            panic!("Invalid integer value for meta property: {}", string);
        }
        result.unwrap()
    }
}
