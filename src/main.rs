#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osh1mc::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::{iter::Cloned, panic::PanicInfo};
use osh1mc::{graphic::GRAPHICS_WRITER, print, println};
use vga::writers::GraphicsWriter;
use x86_64::structures::paging::frame;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use osh1mc::memory;
    use osh1mc::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    use osh1mc::allocator;
    use osh1mc::graphic::{FRAME_BUFFER_HEIGHT, FRAME_BUFFER_WIDTH};
    osh1mc::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed.");
    osh1mc::graphic::init_graphics();
    println!("");
    osh1mc::graphic::TEXT_WRITER.lock().set_color(0x03, 0x16);
    println!("Hello World! {}", 123);
    osh1mc::graphic::TEXT_WRITER.lock().set_color(0x00, 0xff);
    for y in 0..FRAME_BUFFER_HEIGHT as i64 {
        for x in 0..FRAME_BUFFER_WIDTH as i64 {
            let x0 = x - FRAME_BUFFER_WIDTH as i64 / 2;
            let y0 = y - FRAME_BUFFER_HEIGHT as i64 / 2;
            let color = ((x0 * x0 + y0 * y0) / 24 % 0x32) as u8;
            osh1mc::graphic::GRAPHICS_WRITER
                .lock()
                .set_pixel(x as usize, y as usize, color);
        }
    }
    let logo = include_bytes!("data/logo.txt");
    for l in logo {
        print!("{}", *l as char);
    }
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);
    let mut vector = Vec::new();
    for i in 0..500 {
        vector.push(i);
    }
    println!("vector at {:p}", vector.as_slice());
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_refernce = reference_counted.clone();
    println!(
        "current reference count is {}, 0",
        Rc::strong_count(&cloned_refernce)
    );
    core::mem::drop(reference_counted);
    println!(
        "current reference count is {}, 1",
        Rc::strong_count(&cloned_refernce)
    );

    /*
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
    */

    #[cfg(test)]
    test_main();

    println!(
        "Timer: {} sec",
        osh1mc::timer::TIMER.lock().get() as f64 / 10.0
    );
    println!("It did not crash!");
    osh1mc::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use osh1mc::graphic::TEXT_WRITER;
    TEXT_WRITER.lock().set_color(0x01, 0xff);
    println!("{}", info);
    osh1mc::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osh1mc::test_panic_handler(info)
}
