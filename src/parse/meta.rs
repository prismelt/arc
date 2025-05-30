use crate::utilities::color::Color;
use maud::html;

#[derive(Debug, PartialEq)]
pub enum MetaProperties {
    Name(String),
    Title(String),
    FontFamily(String),
    FontSize(u8),
    FontColor(Color),
    BackgroundColor(Color),
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
            crate::warn!("Runtime Warning: Invalid <meta /> property: {}", string);
            return None;
        }
        let key = parts[0].trim();
        let value = parts[1].trim();

        if value.is_empty() || key.is_empty() {
            crate::warn!("Runtime Warning: Invalid <meta /> property: {}", string);
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
                html! { style { "span, a { font-size: " (size) "px; }" } }.into_string()
            }
            MetaProperties::FontColor(color) => {
                html! { style { "span, a { color: " (color.build()) "; }" } }.into_string()
            }
            MetaProperties::BackgroundColor(color) => {
                html! { style { "html, body, main { background-color: " (color.build()) "; }" } }
                    .into_string()
            }
            MetaProperties::PTagFontSize(size) => {
                html! { style { "span, a { font-size: " (size) "px !important; }" } }.into_string()
            }
            MetaProperties::PTagFontColor(color) => {
                html! { style { "span { color: " (color.build()) "; }" } }.into_string()
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
            "name" => Some(MetaProperties::Name(String::from(value))),
            "title" => Some(MetaProperties::Title(String::from(value))),
            "font-family" => Some(MetaProperties::FontFamily(String::from(value))),
            "font-size" => Some(MetaProperties::FontSize(Self::str_to_u8(value)?)),
            "font-color" => Some(MetaProperties::FontColor(Self::str_to_color(value)?)),
            "background-color" => Some(MetaProperties::BackgroundColor(Self::str_to_color(value)?)),
            "text-font-size" => Some(MetaProperties::PTagFontSize(Self::str_to_u8(value)?)),
            "text-color" => Some(MetaProperties::PTagFontColor(Self::str_to_color(value)?)),
            "h1-font-size" => Some(MetaProperties::H1TagFontSize(Self::str_to_u8(value)?)),
            "h1-font-color" => Some(MetaProperties::H1TagFontColor(Self::str_to_color(value)?)),
            "h2-font-size" => Some(MetaProperties::H2TagFontSize(Self::str_to_u8(value)?)),
            "h2-font-color" => Some(MetaProperties::H2TagFontColor(Self::str_to_color(value)?)),
            "h3-font-size" => Some(MetaProperties::H3TagFontSize(Self::str_to_u8(value)?)),
            "h3-font-color" => Some(MetaProperties::H3TagFontColor(Self::str_to_color(value)?)),
            "h4-font-size" => Some(MetaProperties::H4TagFontSize(Self::str_to_u8(value)?)),
            "h4-font-color" => Some(MetaProperties::H4TagFontColor(Self::str_to_color(value)?)),
            _ => {
                crate::warn!("Runtime Warning: Unrecognized <meta /> property: {}", key);
                None
            }
        }
    }

    fn str_to_color(str: &str) -> Option<Color> {
        let result = Color::from_string(str.to_string());
        let Ok(color) = result else {
            crate::warn!("Runtime Warning: {}", result.unwrap_err());
            return None;
        };
        Some(color)
    }

    fn str_to_u8(str: &str) -> Option<u8> {
        let result = str.parse::<u8>();

        let Ok(result) = result else {
            crate::warn!(
                "Runtime Warning: Invalid integer value for meta property: {}",
                str
            );
            return None;
        };
        Some(result)
    }
}
