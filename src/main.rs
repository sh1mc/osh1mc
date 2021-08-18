#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osh1mc::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use osh1mc::println;

#[no_mangle]
#[cfg(not(target_os = "uefi"))]
pub fn _start() -> ! {
    main();
}

// entry point for UEFI
#[cfg(target_os = "uefi")]
#[no_mangle]
pub extern "efiapi" fn efi_main(
    _handle: uefi::Handle,
    stable: uefi::table::SystemTable<uefi::table::Boot>,
) -> uefi::Status {
    use core::fmt::Write;

    stable.stdout().reset(false).unwrap();
    writeln!(stable.stdout(), "hello").unwrap();

    main();

    loop {}
}

fn main() -> ! {
    println!("Hello World! {}", 123);
    osh1mc::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    osh1mc::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    osh1mc::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osh1mc::test_panic_handler(info)
}
