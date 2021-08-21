#![no_std]
#[macro_use]
use crate::video::local_charprint;
pub mod gdt;
use crate::hlt_loop;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pic8259::ChainedPics;
use crate::println;

//static mut tick: u128 = 0;


// Creates the PIC Offsets
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });


// Makes the Interrupt Descriptor Table to use for interrupts
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt
    };
}

// Offset for Pic
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

// Initializes the IDT & PIC
pub fn init_idt() {
    IDT.load();
}

// Timer interrupt handler
// currently doesn't work because we don't have APIC Timer so everytime we run "sti" the system double faults
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    println!(".");
}

// Breakpoint handler
// I have no fucking clue what this does but ok
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    panic!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// In case we double fault
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{

    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}