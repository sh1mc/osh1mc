use lazy_static::lazy_static;
use spin::Mutex;
use vga::writers::{Graphics320x240x256, GraphicsWriter, Screen};
use volatile::Volatile;

const FRAME_BUFFER_HEIGHT: usize = Graphics320x240x256::HEIGHT;
const FRAME_BUFFER_WIDTH: usize = Graphics320x240x256::WIDTH;
const FONT_HEIGHT: usize = 8;
const FONT_WIDTH: usize = 8;
const TEXT_BUFFER_HEIGHT: usize = FRAME_BUFFER_HEIGHT / FONT_HEIGHT;
const TEXT_BUFFER_WIDTH: usize = FRAME_BUFFER_WIDTH / FONT_WIDTH;

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
            let screen_char = &self.text_buffer.chars[TEXT_BUFFER_HEIGHT - row - 1][col];
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
    }
    fn scroll_row(&self) {
        let frame_buffer = GRAPHICS_WRITER.lock().get_frame_buffer();
        for row in 0..(FRAME_BUFFER_HEIGHT - FONT_HEIGHT) {
            for col in 0..(FRAME_BUFFER_WIDTH) {
                unsafe {
                    *((frame_buffer as usize + row * FRAME_BUFFER_WIDTH / 4 + col / 4) as *mut u8) =
                        *((frame_buffer as usize
                            + (row + FONT_HEIGHT) * FRAME_BUFFER_WIDTH / 4
                            + col / 4) as *mut u8)
                };
            }
        }
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
        self.scroll_row();
        self.render_text_buffer_row(0);
    }
}

pub fn init_graphics() {
    GRAPHICS_WRITER.lock().set_mode();
    GRAPHICS_WRITER.lock().clear_screen(0x00);
    /*
    for (offset, character) in "Hello World!".chars().enumerate() {
        GRAPHICS_WRITER
            .lock()
            .draw_character(offset * 8, 0, character, 0xff);
    }
    */
    for _ in 0..20 {
        TEXT_WRITER.lock().write_string("abcdefg\n");
        TEXT_WRITER.lock().write_string("123\n");
    }
    TEXT_WRITER.lock().write_string("It did not crashed!\n");
    //GRAPHICS_WRITER.lock().clear_screen(0xff);
}

#[repr(transparent)]
struct FrameBuffer {
    pixel: [[Volatile<u8>; FRAME_BUFFER_WIDTH]; FRAME_BUFFER_HEIGHT],
}
