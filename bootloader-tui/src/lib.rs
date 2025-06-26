use std::fmt::Write;
use ratatui::backend::{Backend, ClearType, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};
use uefi::boot::ScopedProtocol;
use uefi::{Error as UefiError, Status};
use uefi::proto::console::text::Output;
use crate::color::ToUefiColor;

mod color;
mod error;

#[derive(Default)]
pub struct UefiBackend {
    proto: Option<ScopedProtocol<Output>>,
}

impl UefiBackend {
    pub fn new(proto: ScopedProtocol<Output>) -> Self {
        Self { proto: Some(proto) }
    }
}

impl Backend for UefiBackend {
    type Error = UefiError;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        let Some(proto) = &mut self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        for (x, y, cell) in content {
            proto.set_cursor_position(x as usize, y as usize)?;
            proto.set_color(cell.fg.to_uefi_color(), cell.bg.to_uefi_color())?;
            proto.write_str(cell.symbol()).map_err(|_| UefiError::from(Status::ABORTED))?;
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        let Some(proto) = &mut self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        proto.enable_cursor(false)
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        let Some(proto) = &mut self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        proto.enable_cursor(true)
    }

    fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
        let Some(proto) = &mut self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        let (col, row) = proto.cursor_position();
        Ok(Position::new(col as u16, row as u16))
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), Self::Error> {
        let Some(proto) = &mut self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        let pos = position.into();
        proto.set_cursor_position(pos.x as usize, pos.y as usize)
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        let Some(proto) = &mut self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        proto.clear()
    }

    fn clear_region(&mut self, _: ClearType) -> Result<(), Self::Error> {
        Err(UefiError::from(Status::UNSUPPORTED))
    }

    fn size(&self) -> Result<Size, Self::Error> {
        let Some(proto) = &self.proto else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        let Some(mode) = proto.current_mode()? else {
            return Err(UefiError::from(Status::UNSUPPORTED));
        };
        
        Ok(Size::new(mode.columns() as u16, mode.rows() as u16))
    }

    fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
        Err(UefiError::from(Status::UNSUPPORTED))
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
