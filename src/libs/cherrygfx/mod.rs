// cherrygfx v0.1.1
// a venka v0.1.16 wrapper

pub mod windowserver;

use crate::video::{local_draw_rect, Color, obtain_buffer, Framebuffer, FRAMEBUFFER};
use crate::println;
use spin::{Mutex, MutexGuard};
use conquer_once::spin::OnceCell;
use windowserver::Window;
static WIND_SRV_INFO: OnceCell<Mutex<WindowServerInfo>> = OnceCell::uninit();

pub struct WindowServerInfo {
    palette: Palette,
    w: usize,
    h: usize,
}

// palette struct for making color palette
struct Palette {
    fg: u32,
    bg: u32
}

impl WindowServerInfo {
    pub fn new() -> Self {
        let mut info = Self {
            palette: Palette::new(),
            w: 0,
            h: 0,
        };
        info.w = obtain_framebuffer().width;
        info.h = obtain_framebuffer().height;
        return info;
    }
}

impl Palette {
    pub fn new() -> Self {
        Self {
            fg: 0xFF0000,
            bg: 0x000000
        }
    }
}

//
pub fn init_gfx() {
    let mut ws_info_tmp = WindowServerInfo::new();
    WIND_SRV_INFO.init_once(|| {
        Mutex::new(
            ws_info_tmp
        )
    });
    /*
    write_fgcolor(get_ws_info().palette.fg);
    write_bgcolor(get_ws_info().palette.bg);
     */
    let w_temp = get_ws_info().w;
    let h_temp = get_ws_info().h;
    draw_rect(0, 0, w_temp, h_temp, Color::hex(get_ws_info().palette.bg))
}

pub fn get_ws_info() -> MutexGuard<'static, WindowServerInfo> {
    WIND_SRV_INFO.get().unwrap().lock()
}

// creates a window with said arguments
pub fn create_window(x: usize, y: usize, w: usize, h: usize) -> Window {
    let wind = Window::new(x, y, w, h);
    draw_rect(wind.x, wind.y, wind.w, wind.h, Color::hex(get_ws_info().palette.fg));
    return wind;
}


pub fn obtain_framebuffer() -> MutexGuard<'static, Framebuffer> {
    FRAMEBUFFER.get().unwrap().lock()
}
// simple logger and managment functions
// font render functions
// pub fn gfx_println(string: &str) { local_println(string); }
// gives you a mutable version of the logger to tinker with color
// pub fn gfx_obtainlogger() -> MutexGuard<'static, Logger> { obtain_logger()  }
// writes a different hex color to the foreground of the logger
// pub fn write_fgcolor(hex: u32) { gfx_obtainlogger().fg = hex; }
// writes a different hex color to the background of the logger
// pub fn write_bgcolor(hex: u32) { gfx_obtainlogger().bg = hex; }
// draws a rectangle to the screen
pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, hex: Color) { local_draw_rect(x, y, w, h, hex)}