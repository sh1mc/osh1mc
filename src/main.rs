#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osh1mc::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use osh1mc::{graphic::GRAPHICS_WRITER, print, println};
use vga::writers::{Graphics320x240x256, GraphicsWriter};
//use embedded_graphics_core::

entry_point!(kernel_main);

fn max(x: i32, y: i32) -> i32 {
    let ret;
    if x > y {
        ret = x;
    } else {
        ret = y;
    }
    ret
}

fn abs(x: i32) -> i32 {
    let ret;
    if x < 0 {
        ret = x * -1;
    } else {
        ret = x;
    }
    ret
}

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use osh1mc::memory;
    use osh1mc::memory::BootInfoFrameAllocator;
    use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    /*
    use vga::colors::{Color16, TextModeColor};
    //use vga::writers::{ScreenCharacter, TextWriter, Text80x25};
    let mode = Graphics320x240x256::new();
    mode.set_mode();
    mode.clear_screen(0x00);
    let height = 240;
    let width = 320;
    let frame_buf = mode.get_frame_buffer();
    for y in 0..height {
        for x in 0..width {
            let get_pixel = || {
                unsafe {*(((frame_buf as i32) + (x + y * width) / 4) as *mut u8)}
            };
            let x_fixed = x - width / 2;
            let y_fixed = y - height / 2;
            let set_pixel = |col| {
                unsafe {*(((frame_buf as i32) + (x + y * width) / 4) as *mut u8) = col};
            };
            let col: u8 = if max(abs(x_fixed), abs(y_fixed)) < 50 {0x20} else {get_pixel()};
            set_pixel(col);
            let col: u8 = if x_fixed * x_fixed + y_fixed * y_fixed < 1000 {0x10} else {get_pixel()};
            set_pixel(col);
        }
    }
    mode.draw_line((10, 10), (300, 220), 0xff);
    */

    osh1mc::init();
    osh1mc::graphic::init_graphics();
    println!("");
    osh1mc::graphic::TEXT_WRITER.lock().set_color(0x03, 0x16);
    println!("Hello World! {}", 123);
    osh1mc::graphic::TEXT_WRITER.lock().set_color(0x00, 0xff);
    for y in 0..osh1mc::graphic::FRAME_BUFFER_HEIGHT as i32 {
        for x in 0..osh1mc::graphic::FRAME_BUFFER_WIDTH as i32 {
            let x0 = x - osh1mc::graphic::FRAME_BUFFER_WIDTH as i32 / 2;
            let y0 = y - osh1mc::graphic::FRAME_BUFFER_HEIGHT as i32 / 2;
            let dist = ((x0 * x0 + y0 * y0) / 5 % 0x100) as u8;
            osh1mc::graphic::GRAPHICS_WRITER
                .lock()
                .set_pixel(x as usize, y as usize, dist);
        }
    }
    let logo = include_bytes!("logo.txt");
    for l in logo {
        print!("{}", *l as char);
    }

    /*
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

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
    println!("{}", info);
    osh1mc::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osh1mc::test_panic_handler(info)
}
