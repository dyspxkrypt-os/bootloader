use ratatui::style::Color;
use uefi::proto::console::text::Color as UefiColor;

pub trait ToUefiColor {
    fn to_uefi_color(&self) -> UefiColor;
}

impl ToUefiColor for Color {
    fn to_uefi_color(&self) -> UefiColor {
        match self {
            Color::Black => UefiColor::Black,
            Color::Blue => UefiColor::Blue,
            Color::Green => UefiColor::Green,
            Color::Cyan => UefiColor::Cyan,
            Color::Red => UefiColor::Red,
            Color::Magenta => UefiColor::Magenta,
            Color::DarkGray => UefiColor::DarkGray,
            Color::LightBlue => UefiColor::LightBlue,
            Color::LightGreen => UefiColor::LightGreen,
            Color::LightCyan => UefiColor::LightCyan,
            Color::LightRed => UefiColor::LightRed,
            Color::LightMagenta => UefiColor::LightMagenta,
            Color::Yellow => UefiColor::Yellow,
            Color::White => UefiColor::White,
            _ => UefiColor::White, // unsupported colours
        }
    }
}
