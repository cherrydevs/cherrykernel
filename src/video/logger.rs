use super::{Color, draw_pixel, draw_rect};
use crate::println;

static X_SPACE: usize = 2;
static Y_SPACE: usize = 2;

pub struct Logger {
    pub x: usize,
    pub y: usize,
    char_size: usize,
    bg: u32,
    fg: u32,
    pub multi: usize
}

impl Logger {
    pub fn new() -> Self {
        let logger = Self {
            x: 0,
            y: 0,
            char_size: 8,
            bg: 0x000000,
            fg: 0xFF0000,
            multi: 8
        };
        return logger;
    }
    pub fn set_center(&mut self, width: usize, height: usize) {
        self.x = width / 2;
        self.y = height / 2;
    }
    pub fn line_feed(&mut self) {
        self.y += (self.char_size * self.multi) + (Y_SPACE * self.multi);
        self.x = 0;
    }
    pub fn render_char(&mut self, char: char) {
        self.draw_pixels(self.multi, char);
        self.x += (self.char_size * self.multi) + (X_SPACE * self.multi);
    }
    pub fn println(&mut self, string: &str) {
        let mut index: usize = 0;
        for x in string.chars() {
            //if x == '\\' && temporary_charholder.next() == core::prelude::v1::Some('n') {
            if x == '\n'{
                self.line_feed();
            } else {
                self.render_char(x);
            }
            index += 1;
        }
    }
    fn draw_pixels(&mut self, multi: usize, char: char) {
        //                                                                                                                                                                                    next row                                                                                                                                                          next
        //let a = [Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg)];
        let mut indep_x: usize = 0;
        let mut indep_y: usize = 0;
        let a = self.read_char(char);
        for i in a {
            draw_rect(self.x + indep_x, self.y + indep_y, multi, multi, i);
            if indep_x >= self.char_size * multi {
                indep_x = 0;
                indep_y += 1 * multi;
                if indep_y >= self.char_size * multi {
                    break;
                }
                continue;
            } else {
                indep_x += 1 * multi;
            }
        }
    }
    pub fn read_char(&mut self, char: char) -> [Color; 72] {
        match char {
            'c' => [Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg),Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg)],
            'h' => [Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg),  Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg),  Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg)],
            'e' => [Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg),  Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),  Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),  Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),  Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg),  Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),  Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg),  Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg)],
            'r' => [Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg)],
            'y' => [Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg),  Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg),  Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.bg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg), Color::hex(self.fg)],
            _ => return [Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex(self.fg), Color::hex( self.fg), Color::hex( self.fg), Color::hex( self.fg)]
        }
    }
}