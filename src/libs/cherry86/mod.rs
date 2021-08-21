// cherry86 0.1.5
// wrapper for x86 asm instructions

// halts the os once
pub fn hlt() { unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)); } }
// keeps the os halted until more interrupts come in
pub fn hlt_loop() -> ! { unsafe { loop { hlt(); } } }
// enables interrupts
pub fn intr_enable() { unsafe  { asm!("sti", options(nomem, nostack)) } }
// sends an interrupt to the system, not done yet as you can see
pub fn intr_send(hex: u32) { unsafe { /*asm!("int", options(nomem, nostack))*/ } }