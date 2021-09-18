use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use vga::writers::{Graphics320x240x256, GraphicsWriter, Screen};

pub const FRAME_BUFFER_HEIGHT: usize = Graphics320x240x256::HEIGHT;
pub const FRAME_BUFFER_WIDTH: usize = Graphics320x240x256::WIDTH;
pub const FONT_HEIGHT: usize = 8;
pub const FONT_WIDTH: usize = 8;
pub const TEXT_BUFFER_HEIGHT: usize = FRAME_BUFFER_HEIGHT / FONT_HEIGHT;
pub const TEXT_BUFFER_WIDTH: usize = FRAME_BUFFER_WIDTH / FONT_WIDTH;

lazy_static! {
    pub static ref GRAPHICS_WRITER: Mutex<Graphics320x240x256> =
        Mutex::new(Graphics320x240x256::new());
}

lazy_static! {
    pub static ref TEXT_WRITER: Mutex<TextWriter> = Mutex::new(TextWriter {
        column_pos: 0,
        bg_color: 0x00,
        fg_color: 0xff,
        text_buffer: TextBuffer {
            chars: [[ScreenChar::new(); TEXT_BUFFER_WIDTH]; TEXT_BUFFER_HEIGHT]
        },
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScreenChar {
    character: char,
    bg_color: u8,
    fg_color: u8,
}

impl ScreenChar {
    fn new() -> Self {
        Self {
            character: ' ',
            bg_color: 0x00,
            fg_color: 0xff,
        }
    }
}

struct TextBuffer {
    chars: [[ScreenChar; TEXT_BUFFER_WIDTH]; TEXT_BUFFER_HEIGHT],
}

pub struct TextWriter {
    text_buffer: TextBuffer,
    column_pos: usize,
    bg_color: u8,
    fg_color: u8,
}

impl TextWriter {
    pub fn new() {}
    pub fn set_color(&mut self, bg: u8, fg: u8) {
        self.bg_color = bg;
        self.fg_color = fg;
    }
    pub fn write_char(&mut self, character: char) {
        match character {
            '\n' => self.new_line(),
            character => {
                if self.column_pos >= TEXT_BUFFER_WIDTH {
                    self.new_line();
                }
                let row = TEXT_BUFFER_HEIGHT - 1;
                let col = self.column_pos;
                self.text_buffer.chars[row][col] = ScreenChar {
                    character: character,
                    bg_color: self.bg_color,
                    fg_color: self.fg_color,
                };
                self.render_text_buffer_col(0, col);
                self.column_pos += 1;
            }
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for character in s.chars() {
            self.write_char(character);
        }
    }
    pub fn render_text_buffer(&mut self) {
        for row in 0..TEXT_BUFFER_HEIGHT {
            self.render_text_buffer_row(row);
        }
    }
    fn render_text_buffer_row(&mut self, row: usize) {
        for col in 0..TEXT_BUFFER_WIDTH {
            self.render_text_buffer_col(row, col);
        }
    }
    fn render_text_buffer_col(&mut self, row: usize, col: usize) {
        let screen_char = &self.text_buffer.chars[TEXT_BUFFER_HEIGHT - row - 1][col];
        if row < TEXT_BUFFER_HEIGHT - 1 {
            let old_screen_char = &self.text_buffer.chars[TEXT_BUFFER_HEIGHT - row - 2][col];
            if screen_char == old_screen_char && row != 0 {
                return;
            }
        }
        for y in 0..FONT_HEIGHT {
            for x in 0..FONT_WIDTH {
                GRAPHICS_WRITER.lock().set_pixel(
                    col * FONT_WIDTH + x,
                    (TEXT_BUFFER_HEIGHT - row - 1) * FONT_HEIGHT + y,
                    screen_char.bg_color,
                );
            }
        }
        GRAPHICS_WRITER.lock().draw_character(
            col * FONT_WIDTH,
            (TEXT_BUFFER_HEIGHT - row - 1) * FONT_HEIGHT,
            screen_char.character,
            screen_char.fg_color,
        );
    }
    fn new_line(&mut self) {
        self.render_text_buffer_row(0);
        for row in 1..TEXT_BUFFER_HEIGHT {
            for col in 0..TEXT_BUFFER_WIDTH {
                self.text_buffer.chars[row - 1][col] = self.text_buffer.chars[row][col];
            }
        }
        for col in 0..TEXT_BUFFER_WIDTH {
            self.text_buffer.chars[TEXT_BUFFER_HEIGHT - 1][col] = ScreenChar::new();
        }
        self.column_pos = 0;
        self.render_text_buffer();
    }
}

pub fn init_graphics() {
    use crate::println_info;
    GRAPHICS_WRITER.lock().set_mode();
    GRAPHICS_WRITER.lock().clear_screen(0x00);
    println_info!("VGA Initialized.");
}

impl fmt::Write for TextWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        TEXT_WRITER.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::graphic::_print(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_info {
    ($($arg:tt)*) => (
        $crate::print!("[");
        $crate::graphic::TEXT_WRITER.lock().set_color(0x00, 0x10);
        $crate::print!("ok");
        $crate::graphic::TEXT_WRITER.lock().set_color(0x00, 0xff);
        $crate::print!("] {}\n", format_args!($($arg)*));
    );
}
