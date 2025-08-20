# rappyd - A Modular Bootloader

`rappyd` is a work-in-progress modular bootloader designed to be adaptable for various kernels and operating systems. This project is currently in the initial development phase.

## Current Status

The bootloader is implemented as a UEFI application written in Rust. It currently initializes the UEFI environment and displays a welcome message. The codebase has been refactored for modularity, with I/O functions separated into their own module.

## Future Development

The next major step is to implement filesystem support (e.g., FAT32) to locate and load a kernel file from disk.

### Low-Level Implementation

It is acknowledged that for certain low-level operations, direct use of assembly language will be necessary. Future phases of development will integrate assembly code for tasks such as:
*   Direct CPU register manipulation.
*   Handling hardware interrupts and exceptions.
*   Performance-critical context switching.

This hybrid Rust/Assembly approach will allow for both high-level safety and low-level control.
