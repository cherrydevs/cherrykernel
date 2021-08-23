#![no_std]
#![no_main]
#![feature(asm)]
#[macro_use]

extern crate alloc;

use cherrykernel;
use cherrykernel::{init_int_extern, println, hlt_loop, allocator::memory::{active_level_4_table, BootInfoFrameAllocator}};
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;
use cherrykernel::allocator;
use cherrykernel::video::init_gop;
use cherrykernel::libs::{cherrygfx};

entry_point!(kernel_main);

// note to self, map apic so no double fault

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    //Initializes the kernel, Interrupts, Serial, Graphics, etc...
    println!("CherryKernel Serial Output");
    init_int_extern();
    // Initializes Graphics and the corresponding library
    init_gop(boot_info.framebuffer.as_mut().unwrap());
    cherrygfx::init_gfx();
    //gfx_println("cherry by limey");
    cherrygfx::create_window(300, 300, 100, 100);
    /*
    init_memory(boot_info);
    gfx_obtainlogger().fg = 0xFFFFFF;
    gfx_println("abcdefghijklmnop\nqrstuvwxyz\n\n\n\n\n\n\n\n\n");
    gfx_obtainlogger().fg = 0xFF0000;
    gfx_println("cherry\n");
    gfx_obtainlogger().fg = 0x00FF00;
    gfx_println("by limeyteam");
    */


    unsafe {

    }
    //x86_64::instructions::interrupts::enable();
    /*
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
    */

    //x86_64::instructions::interrupts::int3();
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