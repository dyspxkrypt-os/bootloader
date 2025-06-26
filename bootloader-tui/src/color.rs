use ratatui::style::Color;

pub trait ToUefiColor {
    fn to_uefi_color(&self) -> usize;
}

impl ToUefiColor for Color {
    fn to_uefi_color(&self) -> usize {
        match self {
            Color::Black => 0x00,
            Color::Blue => 0x01,
            Color::Green => 0x02,
            Color::Cyan => 0x03,
            Color::Red => 0x04,
            Color::Magenta => 0x05,
            Color::DarkGray => 0x08,
            Color::LightBlue => 0x09,
            Color::LightGreen => 0x0A,
            Color::LightCyan => 0x0B,
            Color::LightRed => 0x0C,
            Color::LightMagenta => 0x0D,
            Color::Yellow => 0x0E,
            Color::White => 0x0F,
            _ => 0x00, // unsupported colours
        }
    }
}

impl ToUefiColor for (Color, Color) {
    fn to_uefi_color(&self) -> usize {
        let (fg, bg) = self;
        (bg.to_uefi_color() << 4) | fg.to_uefi_color()
    }
}
