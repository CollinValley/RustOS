#![no_std]
#![feature(abi_x86_interrupt)]

extern crate bootloader_precompiled;
extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;
extern crate uart_16550;
extern crate x86_64;

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate std;

// We need to add `pub` here to make them accessible from the outside
#[macro_use]
pub mod vga_buffer;
pub mod serial;
pub mod interrupts; //must be after vga_buffer since we use its print macros
pub mod gdt;

/// Exit Function
///
/// Exits QEMU by writing a 0 to a particular port
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
