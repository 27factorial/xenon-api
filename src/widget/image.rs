use std::mem::MaybeUninit;

use super::Widget;
use crate::ffi;
use embedded_graphics::prelude::Point;
use thiserror::Error;

const IMAGE_COLOR_BLACK: u8 = 0b00;
const IMAGE_COLOR_WHITE: u8 = 0b01;
const IMAGE_COLOR_TRANSPARENT: u8 = 0b11;

fn wasm_to_bitmap_error(err: i32, e1: u32, e2: u32) -> Option<BitmapError> {
    match err {
        -1 => Some(BitmapError::NoWidth),
        -2 => Some(BitmapError::NoHeight),
        -3 => Some(BitmapError::InvalidDimensions {
            width: e1 as u8,
            height: e2 as u8,
        }),
        -4 => Some(BitmapError::LengthMismatch {
            expected: e1 as usize,
            actual: e2 as usize,
        }),
        -5 => Some(BitmapError::DecompressionFailed),
        _ => None,
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CompressedBitmap {
    pub position: Point,
    width: u8,
    height: u8,
    id: i32,
}

impl CompressedBitmap {
    pub fn new(position: Point, width: u8, height: u8, bytes: &[u8]) -> Self {
        let id = unsafe { ffi::widget::load_compressed_bitmap(bytes.as_ptr(), bytes.len()) };

        Self {
            position,
            width,
            height,
            id,
        }
    }

    pub fn from_encoded(position: Point, bytes: &[u8]) -> Result<Self, BitmapError> {
        let mut iter = bytes.iter();

        let &width = iter.next().ok_or(BitmapError::NoWidth)?;
        let &height = iter.next().ok_or(BitmapError::NoHeight)?;

        let data = iter.as_slice();

        let id = unsafe { ffi::widget::load_compressed_bitmap(data.as_ptr(), data.len()) };

        Ok(Self {
            position,
            width,
            height,
            id,
        })
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn decompress(self) -> Result<Bitmap, BitmapError> {
        let mut e1 = 0;
        let mut e2 = 0;

        let err = unsafe {
            ffi::widget::decompress_bitmap(self.id, self.width, self.height, &mut e1, &mut e2)
        };

        match wasm_to_bitmap_error(err, e1, e2) {
            Some(e) => Err(e),
            None => Ok(Bitmap {
                position: self.position,
                width: self.width,
                height: self.height,
                id: self.id,
            }),
        }
    }
}

impl Widget for CompressedBitmap {
    fn render(&self) {
        unsafe {
            ffi::widget::draw_compressed_bitmap(
                self.id,
                self.width,
                self.height,
                self.position.x,
                self.position.y,
            );
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Bitmap {
    pub position: Point,
    width: u8,
    height: u8,
    id: i32,
}

impl Bitmap {
    pub fn new(
        position: Point,
        width: u8,
        height: u8,
        data: impl AsRef<[u8]>,
    ) -> Result<Self, BitmapError> {
        let data = data.as_ref();

        let mut e1 = 0;
        let mut e2 = 0;

        let id =
            unsafe { ffi::widget::load_bitmap(width, height, data.as_ptr(), &mut e1, &mut e2) };

        match wasm_to_bitmap_error(id, e1, e2) {
            Some(e) => Err(e),
            None => Ok(Self {
                position,
                width,
                height,
                id,
            }),
        }
    }

    pub fn from_encoded(position: Point, encoded: &[u8]) -> Result<Self, BitmapError> {
        let mut iter = encoded.iter();

        let &width = iter.next().ok_or(BitmapError::NoWidth)?;
        let &height = iter.next().ok_or(BitmapError::NoHeight)?;

        Self::new(position, width, height, iter.as_slice())
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, color: PixelColor) {
        let wasm_pixel = color.to_wasm();

        unsafe { ffi::widget::set_bitmap_pixel(self.id, self.width, self.height, x, y, wasm_pixel) }
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> Option<PixelColor> {
        unsafe {
            let val = ffi::widget::get_bitmap_pixel(self.id, self.width, self.height, x, y);
            PixelColor::from_wasm(val)
        }
    }
}

impl Widget for Bitmap {
    fn render(&self) {
        unsafe {
            ffi::widget::draw_bitmap(
                self.id,
                self.width,
                self.height,
                self.position.x,
                self.position.y,
            );
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
pub enum BitmapError {
    #[error("no width specified")]
    NoWidth,
    #[error("no height specified")]
    NoHeight,
    #[error("invalid dimensions ({width}x{height})")]
    InvalidDimensions { width: u8, height: u8 },
    #[error("length mismatch, expected {expected} got {actual}")]
    LengthMismatch { expected: usize, actual: usize },
    #[error("decompression error")]
    DecompressionFailed,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PixelColor {
    Black = IMAGE_COLOR_BLACK,
    White = IMAGE_COLOR_WHITE,
    Transparent = IMAGE_COLOR_TRANSPARENT,
}

impl PixelColor {
    fn to_wasm(self) -> u32 {
        match self {
            Self::Black => 1,
            Self::White => 2,
            Self::Transparent => 3,
        }
    }

    fn from_wasm(val: u32) -> Option<Self> {
        match val {
            1 => Some(Self::Black),
            2 => Some(Self::White),
            3 => Some(Self::Transparent),
            _ => None,
        }
    }
}
