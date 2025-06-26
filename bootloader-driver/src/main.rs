#![no_main]
#![no_std]

mod app;

use crate::app::BootloaderApp;
use bootloader_tui::UefiBackend;
use ratatui::Terminal;
use uefi::boot::{image_handle, open_protocol_exclusive};
use uefi::proto::console::text::Output;
use uefi::{Status, entry};

#[entry]
pub fn main() -> Status {
    let handle = image_handle();
    let proto = open_protocol_exclusive::<Output>(handle).unwrap();
    let tui_backend = UefiBackend::new(proto);

    let mut terminal = Terminal::new(tui_backend).unwrap();
    if BootloaderApp::default().run(&mut terminal).is_err() {
        return Status::ABORTED;
    }

    Status::SUCCESS
}
