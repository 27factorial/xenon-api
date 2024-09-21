use std::convert::Infallible;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, OriginDimensions, Size},
    Drawable, Pixel,
};

use crate::ffi::{self, widget::clear_buffer};

pub mod button;
pub mod collections;
mod eg;
pub mod image;
pub mod misc;

pub const BUFFER_WIDTH: u8 = 144;
pub const BUFFER_HEIGHT: u8 = 168;
pub const BUFFER_SIZE: usize = (BUFFER_WIDTH as usize * BUFFER_HEIGHT as usize) / 8;
const BYTES_PER_LINE: usize = BUFFER_WIDTH as usize / 8;

pub fn clear() {
    unsafe {
        clear_buffer();
    }
}

pub trait Widget {
    fn render(&self);
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct WidgetBuffer {
    buf: [u8; BUFFER_SIZE],
    min_changed: u8,
    max_changed: u8,
    clear: bool,
}

impl WidgetBuffer {
    pub const fn new() -> Self {
        Self {
            buf: [0xff; BUFFER_SIZE],
            min_changed: BUFFER_HEIGHT,
            max_changed: 0,
            clear: false,
        }
    }

    pub fn fill(&mut self, byte: u8) {
        self.buf.fill(byte);
        self.min_changed = 0;
        self.max_changed = BUFFER_HEIGHT;
    }

    pub fn clear(&mut self) {
        self.buf.fill(0xff);
        self.min_changed = BUFFER_HEIGHT;
        self.max_changed = 0;
        self.clear = true;
    }

    pub fn set_pixel<T>(&mut self, x: T, y: T, color: BinaryColor)
    where
        T: TryInto<u8>,
    {
        let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) else {
            // If it's out of range for u8, it's obviously out of range
            // for the width or height of the LCD.
            return;
        };

        self.set_pixel_internal(x, y, color)
    }

    pub(crate) fn set_pixel_internal(&mut self, x: u8, y: u8, color: BinaryColor) {
        const LCD_WHITE_LUT: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
        const LCD_BLACK_LUT: [u8; 8] = [!1, !2, !4, !8, !16, !32, !64, !128];

        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            let (index, bit) = Self::get_index_and_bit(x, y);

            if color.is_on() {
                self.buf[index] &= LCD_BLACK_LUT[bit];
            } else {
                self.buf[index] |= LCD_WHITE_LUT[bit];
            }

            self.min_changed = self.min_changed.min(y);
            self.max_changed = self.max_changed.max(y.saturating_add(1)).min(BUFFER_HEIGHT);
            self.clear = false;
        }
    }

    pub fn copy_from_buffer(&mut self, other: &Self) {
        *self = *other
    }

    pub fn get_line(&self, n: usize) -> &[u8] {
        let idx = n * BYTES_PER_LINE;

        &self.buf[idx..idx + BYTES_PER_LINE]
    }

    pub fn get_line_mut(&mut self, n: usize) -> &mut [u8] {
        let idx = n * BYTES_PER_LINE;

        &mut self.buf[idx..idx + BYTES_PER_LINE]
    }

    pub fn refreshed(&mut self) {
        self.min_changed = BUFFER_HEIGHT;
        self.max_changed = 0;
        self.clear = false;
    }

    pub fn needs_refresh(&self) -> bool {
        self.min_changed != BUFFER_HEIGHT && self.max_changed != 0
    }

    pub fn needs_clear(&self) -> bool {
        self.clear
    }

    #[inline]
    const fn get_index_and_bit(x: u8, y: u8) -> (usize, usize) {
        let index = (x as usize + BUFFER_WIDTH as usize * y as usize) >> 3;
        let bit = (x & 7) as usize;

        (index, bit)
    }
}

impl Default for WidgetBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl OriginDimensions for WidgetBuffer {
    fn size(&self) -> Size {
        Size::new(BUFFER_WIDTH as u32, BUFFER_HEIGHT as u32)
    }
}

impl DrawTarget for WidgetBuffer {
    type Color = BinaryColor;

    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            self.set_pixel(point.x, point.y, color);
        }

        Ok::<_, Infallible>(())
    }
}
