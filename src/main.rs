#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(xm_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use xm_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    xm_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    xm_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xm_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xm_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
