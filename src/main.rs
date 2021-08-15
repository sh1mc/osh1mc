#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osh1mc::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osh1mc::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! {}", 123);
    osh1mc::init();

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osh1mc::test_panic_handler(info)
}
