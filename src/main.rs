#![no_std]   // don't link the Rust standard library
#![cfg_attr(not(test), no_main)]  // disable all Rust-level entry points, unless testing
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))] //silence warnings for testing

#[macro_use]
extern crate blog_os;
extern crate x86_64;

use core::panic::PanicInfo;

/// _start function for linux machines
#[cfg(not(test))]
#[no_mangle] //don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::gdt::init();
    blog_os::interrupts::init_idt();

    // invoke a breakpoint exception
    // x86_64::instructions::int3();

    // trigger a page fault
//    unsafe {
 //       *(0xdeadbeef as *mut u64) = 42; //scary
  //  };

    println!("It did not crash!");
    loop {}
}

/// This function is called on panic
#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
