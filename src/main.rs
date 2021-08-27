#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osh1mc::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use osh1mc::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use osh1mc::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World! {}", 123);
    osh1mc::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        use x86_64::structures::paging::PageTable;
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }

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
