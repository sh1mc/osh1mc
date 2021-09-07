#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osh1mc::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use vga::writers::{Graphics320x240x256, Graphics640x480x16, GraphicsWriter};
use core::panic::PanicInfo;
use osh1mc::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use osh1mc::memory;
    use osh1mc::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    use vga::colors::{Color16, TextModeColor};
    //use vga::writers::{ScreenCharacter, TextWriter, Text80x25};
    let mode = Graphics320x240x256::new();
    mode.set_mode();
    mode.clear_screen(0x00);
    mode.draw_line((10, 10), (300, 220), 0xff);

    println!("Hello World! {}", 123);
    osh1mc::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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
