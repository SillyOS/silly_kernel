#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(silly_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use silly_os::*;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    silly_os::init();
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    silly_os::test_panic_handler(info)
}
