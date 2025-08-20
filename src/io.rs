use core::fmt::Write;
use uefi::prelude::*;

pub fn println(system_table: &mut SystemTable<Boot>) {
    writeln!(system_table.stdout(), "Welcome to rappyd bootloader!").expect("Failed to write to stdout");
}
