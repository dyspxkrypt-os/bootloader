use crate::color::ToUefiColor;
use crate::error::UefiStopError;
use r_efi::base::Boolean;
use r_efi::protocols::simple_text_output::Protocol as SimpleTextOutputProtocol;
use ratatui::backend::{Backend, ClearType, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};

mod color;
mod error;

pub struct UefiStopBackend {
    stop: SimpleTextOutputProtocol,
}

impl UefiStopBackend {
    pub fn new(stop: SimpleTextOutputProtocol) -> UefiStopBackend {
        Self { stop }
    }
}

impl Backend for UefiStopBackend {
    type Error = UefiStopError;

    fn draw<'a, I>(&mut self, content: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, cell) in content {
            let mut status =
                (self.stop.set_cursor_position)(&mut self.stop, x as usize, y as usize);
            if status.is_error() {
                return Err(UefiStopError(status));
            }

            let color = (cell.fg, cell.bg).to_uefi_color();
            status = (self.stop.set_attribute)(&mut self.stop, color);
            if status.is_error() {
                return Err(UefiStopError(status));
            }

            let bytes = cell.symbol().as_bytes();
            let mut buf = [0u16; 2];
            buf[0] = bytes[0] as u16;
            buf[1] = 0;

            status = (self.stop.output_string)(&mut self.stop, buf.as_mut_ptr());
            if status.is_error() {
                return Err(UefiStopError(status));
            }
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        let status = (self.stop.enable_cursor)(&mut self.stop, Boolean::FALSE);
        if status.is_error() {
            return Err(UefiStopError(status));
        }

        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        let status = (self.stop.enable_cursor)(&mut self.stop, Boolean::TRUE);
        if status.is_error() {
            return Err(UefiStopError(status));
        }

        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<Position, Self::Error> {
        unsafe {
            Ok(Position::new(
                (*self.stop.mode).cursor_row as u16,
                (*self.stop.mode).cursor_column as u16,
            ))
        }
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> Result<(), Self::Error> {
        let pos = position.into();

        let status = (self.stop.set_cursor_position)(&mut self.stop, pos.x as usize, pos.y as usize);
        if status.is_error() {
            return Err(UefiStopError(status));
        }

        Ok(())
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        let status = (self.stop.clear_screen)(&mut self.stop);
        if status.is_error() {
            return Err(UefiStopError(status));
        }

        Ok(())
    }

    fn clear_region(&mut self, _: ClearType) -> Result<(), Self::Error> {
        unimplemented!("clear region is unsupported for this backend");
    }

    fn size(&self) -> Result<Size, Self::Error> {
        let _ = (self.stop.query_mode)(&mut self.stop, )
        
        todo!()
    }

    fn window_size(&mut self) -> Result<WindowSize, Self::Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
