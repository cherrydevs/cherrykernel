#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]
#![feature(destructuring_assignment)]
#![allow(incomplete_features)]
#![feature(const_generics)]
#[feature(asm)]

extern crate alloc;

pub mod serial;
pub mod allocator;
pub mod video;
pub mod interrupts;
pub mod libs;

use interrupts::PICS;
use x86_64;
use core::panic::PanicInfo;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, BootInfo};
use allocator::memory::active_level_4_table;
use x86_64::VirtAddr;
use allocator::memory::BootInfoFrameAllocator;
use alloc::alloc::Layout;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("!!! PANIC !!!\n{}", info);
    hlt_loop();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init_int_extern() {
    interrupts::gdt::init_gdt();
    interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    //x86_64::instructions::interrupts::enable();
}

#[macro_export]
macro_rules! println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}