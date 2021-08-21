// venka v0.1.16 wrapper
use crate::video::{local_println, obtain_logger, logger::Logger};
use spin::{Mutex, MutexGuard};

// font render functions
pub fn gfx_println(string: &str) {
    local_println(string);
}

pub fn gfx_obtainlogger() -> MutexGuard<'static, Logger> {
    obtain_logger()
}