#![no_std]
#![no_main]
#[macro_use]
#[feature(asm)]

extern crate alloc;

use cherrykernel;
use cherrykernel::{println, hlt_loop, allocator::memory::{active_level_4_table, BootInfoFrameAllocator}};
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
use cherrykernel::allocator;
use cherrykernel::video::init_gop;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Initializes the kernel, Interrupts, Serial, Graphics, etc...
    println!("CherryKernel Serial Output");
    init_gop(boot_info.framebuffer.as_mut().unwrap());
    init_memory(boot_info);
    hlt_loop();
    loop{}
}

fn init_memory(boot_info: &'static BootInfo) {
    use crate::BootInfoFrameAllocator;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { allocator::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}