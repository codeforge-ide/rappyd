#![no_main]
#![no_std]

// These extern crate lines are not strictly necessary in Rust 2018 edition,
// but they are helpful for clarity in a `no_std` environment.
extern crate uefi;
extern crate uefi_services;

mod io;

use uefi::prelude::*;

#[entry]
fn efi_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize the UEFI services. This must be done before using any of the
    // services, and it can only be done once.
    uefi_services::init(&mut system_table).expect("Failed to initialize UEFI services");

    // Clear the screen
    system_table
        .stdout()
        .reset(false)
        .expect("Failed to reset stdout");

    // Print our welcome message.
    io::println(&mut system_table);

    // Loop forever to prevent the firmware from exiting the application.
    loop {}
}
