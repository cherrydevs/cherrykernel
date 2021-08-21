// cherrygfx 0.1.1
// a venka v0.1.16 wrapper
use crate::video::{local_println, obtain_logger, logger::Logger};
use spin::{Mutex, MutexGuard};

// font render functions
pub fn gfx_println(string: &str) { local_println(string); }
// gives you a mutable version of the logger to tinker with color
pub fn gfx_obtainlogger() -> MutexGuard<'static, Logger> { obtain_logger()  }
// writes a different hex color to the foreground of the logger
pub fn write_fgcolor(hex: u32) { gfx_obtainlogger().fg = hex; }
// writes a different hex color to the background of the logger
pub fn write_bgcolor(hex: u32) { gfx_obtainlogger().bg = hex; }