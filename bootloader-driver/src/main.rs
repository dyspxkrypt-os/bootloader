#![no_main]
#![no_std]

use r_efi::base::Handle;
use r_efi::efi::SystemTable;

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(handle: Handle, system_table: *const SystemTable) {}
