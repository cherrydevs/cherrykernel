// Based off https://github.com/anellie/yacuri/blob/main/kernel/src/graphics/mod.rs
#![no_std]
#![feature(alloc)]
extern crate alloc;
use alloc::slice;
pub mod logger;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use spin::{Mutex, MutexGuard};
use conquer_once::spin::OnceCell;
use crate::println;
static FRAMEBUFFER: OnceCell<Mutex<Framebuffer>> = OnceCell::uninit();
use spinning_top::Spinlock;
use core::{
    fmt::{self, Write},
    ptr,
};

pub fn init_gop(mut buffer: &mut FrameBuffer) {
    // Play with the borrow checker a bit to get a raw frame buffer
    // with 'static lifetime
    let FrameBufferInfo {
        horizontal_resolution: width,
        vertical_resolution: height,
        stride,
        bytes_per_pixel,
        ..
    } = buffer.info();
    let buffer_ptr = buffer.buffer_mut().as_mut_ptr();
    let buffer_len = buffer.buffer_mut().len();

    FRAMEBUFFER.init_once(|| {
        Mutex::new(Framebuffer {
            buffer: unsafe { slice::from_raw_parts_mut(buffer_ptr, buffer_len) },
            height,
            width,
            stride: stride * bytes_per_pixel,
            bytes_per_pixel,
        })
    });
    let bg = Color::hex(0x000000);
    let mut logger_instance = logger::Logger::new();
    println!("{:#?}", buffer.info());
    draw_rect(0, 0, width, height, bg);
    logger_instance.println("cherry\ncherry\ncherry\ncherry\ncherry\ncherry");
    /*
    logger_instance.render_char('c');
    logger_instance.render_char('c');
    //logger_instance.line_feed();
    logger_instance.multi = 16;
    logger_instance.render_char('c');
    logger_instance.render_char('c');
    logger_instance.multi = 8;
    logger_instance.render_char('c');
    logger_instance.render_char('c');
    */
}

pub struct Framebuffer {
    // the underlying buffer
    buffer: &'static mut [u8],
    // height in pixels
    height: usize,
    // width in pixels
    width: usize,
    // stride in bytes (!!)
    stride: usize,
    // bytes per pixel
    bytes_per_pixel: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn from(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub fn hex(hex: u32) -> Color {
        Color {
            red: (hex >> 16) as u8,
            green: (hex >> 8) as u8,
            blue: hex as u8,
        }
    }
}

fn obtain_buffer() -> MutexGuard<'static, Framebuffer> {
    FRAMEBUFFER.get().unwrap().lock()
}

fn draw_pixel(x: usize, y: usize, color: Color) {
    let mut buf = obtain_buffer();
    let offset = y * buf.stride + (x * buf.bytes_per_pixel);
    set_pixel(buf.buffer, offset, color)
}

fn draw_hori_line(x: usize, y: usize, len: usize, color: Color) {
    let mut buf = obtain_buffer();
    assert!((x + len) <= buf.width);
    let mut offset = y * buf.stride + (x * buf.bytes_per_pixel);
    for _ in 0..len {
        set_pixel(buf.buffer, offset, color);
        offset += buf.bytes_per_pixel;
    }
}

pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, color: Color) {
    let mut buf = obtain_buffer();
    assert!((x + w) <= buf.width);
    assert!((y + h) <= buf.width);

    let mut line_offset = y * buf.stride + (x * buf.bytes_per_pixel);
    let mut offset = line_offset;
    for _ in 0..h {
        for _ in 0..w {
            set_pixel(buf.buffer, offset, color);
            offset += buf.bytes_per_pixel;
        }
        line_offset += buf.stride;
        offset = line_offset;
    }
}

#[inline]
fn set_pixel(buf: &mut [u8], offset: usize, color: Color) {
    buf[offset] = color.blue;
    buf[offset + 1] = color.green;
    buf[offset + 2] = color.red;
}

