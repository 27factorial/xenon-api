use crate::syscalls;

use super::tuple::Tuple3;

syscalls! {
    // Primitives
    pub fn draw_arc(
        top_left_x: i32,
        top_left_y: i32,
        diameter: u32,
        angle_start: f32,
        angle_sweep: f32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32
    );
    pub fn draw_circle(
        top_left_x: i32,
        top_left_y: i32,
        diameter: u32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32
    );
    pub fn draw_ellipse(
        top_left_x: i32,
        top_left_y: i32,
        width: u32,
        height: u32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32,
    );
    pub fn draw_line(
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32,
    );
    pub fn draw_rectangle(
        top_left_x: i32,
        top_left_y: i32,
        width: u32,
        height: u32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32,
    );
    pub fn draw_rounded_rectangle(
        top_left_x: i32,
        top_left_y: i32,
        width: u32,
        height: u32,
        corners_ptr: *const u32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32,
    );
    pub fn draw_sector(
        top_left_x: i32,
        top_left_y: i32,
        diameter: u32,
        angle_start: f32,
        angle_sweep: f32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32
    );
    pub fn draw_triangle(
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        fill_color: u32,
        stroke_color: u32,
        stroke_width: u32,
        stroke_alignment: u32,
    );

    // Bitmaps
    pub fn load_compressed_bitmap(
        ptr: *const u8,
        len: usize,
    ) -> i32;
    pub fn load_bitmap(
        width: u8,
        height: u8,
        ptr: *const u8,
        e1_ptr: *mut u32,
        e2_ptr: *mut u32,
    ) -> i32;
    pub fn decompress_bitmap(
        id: i32,
        width: u8,
        height: u8,
        e1_ptr: *mut u32,
        e2_ptr: *mut u32,
    ) -> i32;
    pub fn draw_compressed_bitmap(
        id: i32,
        width: u8,
        height: u8,
        x: i32,
        y: i32,
    );
    pub fn draw_bitmap(
        id: i32,
        width: u8,
        height: u8,
        x: i32,
        y: i32,
    );
    pub fn get_bitmap_pixel(
        id: i32,
        width: u8,
        height: u8,
        x: u8,
        y: u8,
    ) -> u32;
    pub fn set_bitmap_pixel(
        id: i32,
        width: u8,
        height: u8,
        x: u8,
        y: u8,
        pixel_color: u32,
    );

    pub fn clear_buffer();
}
