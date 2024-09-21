use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Angle, Point, Size},
    primitives::{
        Arc, Circle, CornerRadii, CornerRadiiBuilder, Ellipse, Line, PrimitiveStyle,
        PrimitiveStyleBuilder, Rectangle, RoundedRectangle, Sector, StrokeAlignment, Styled,
        Triangle,
    },
};

use crate::ffi;

use super::Widget;

type Primitive<T> = Styled<T, PrimitiveStyle<BinaryColor>>;

macro_rules! style_to_wasm {
    ($fill_color:expr, $stroke_color:expr, $stroke_alignment:expr) => {{
        let fill_color = color_to_wasm($fill_color);
        let stroke_color = color_to_wasm($stroke_color);
        let stroke_align = stroke_align_to_wasm($stroke_alignment);

        (fill_color, stroke_color, stroke_align)
    }};
}

fn color_to_wasm(color: Option<BinaryColor>) -> u32 {
    match color {
        None => 0,
        Some(BinaryColor::Off) => 1,
        Some(BinaryColor::On) => 2,
    }
}

fn stroke_align_to_wasm(align: StrokeAlignment) -> u32 {
    match align {
        StrokeAlignment::Inside => 0,
        StrokeAlignment::Center => 1,
        StrokeAlignment::Outside => 2,
    }
}

impl Widget for Primitive<Arc> {
    fn render(&self) {
        let Arc {
            top_left,
            diameter,
            angle_start,
            angle_sweep,
        } = self.primitive;
        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let angle_start = angle_start.to_radians();
        let angle_sweep = angle_sweep.to_radians();

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_arc(
                top_left.x,
                top_left.y,
                diameter,
                angle_start,
                angle_sweep,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<Circle> {
    fn render(&self) {
        let Circle { top_left, diameter } = self.primitive;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_circle(
                top_left.x,
                top_left.y,
                diameter,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<Ellipse> {
    fn render(&self) {
        let Ellipse { top_left, size } = self.primitive;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_ellipse(
                top_left.x,
                top_left.y,
                size.width,
                size.height,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<Line> {
    fn render(&self) {
        let Line { start, end } = self.primitive;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<Rectangle> {
    fn render(&self) {
        let Rectangle { top_left, size } = self.primitive;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_rectangle(
                top_left.x,
                top_left.y,
                size.width,
                size.height,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<RoundedRectangle> {
    fn render(&self) {
        let RoundedRectangle { rectangle, corners } = self.primitive;

        let Rectangle { top_left, size } = rectangle;

        let CornerRadii {
            top_left: ctl,
            top_right: ctr,
            bottom_right: cbr,
            bottom_left: cbl,
        } = corners;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        #[rustfmt::skip]
        let corner_array = [
            // Top left
            ctl.width, ctl.height,
            // Top Right
            ctr.width, ctr.height,
            // Bottom Right
            cbr.width, cbr.height,
            // Bottom Left
            cbl.width, cbl.height,
        ];

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_rounded_rectangle(
                top_left.x,
                top_left.y,
                size.width,
                size.height,
                corner_array.as_ptr(),
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<Sector> {
    fn render(&self) {
        let Sector {
            top_left,
            diameter,
            angle_start,
            angle_sweep,
        } = self.primitive;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let angle_start = angle_start.to_radians();
        let angle_sweep = angle_sweep.to_radians();

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_sector(
                top_left.x,
                top_left.y,
                diameter,
                angle_start,
                angle_sweep,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}

impl Widget for Primitive<Triangle> {
    fn render(&self) {
        let Triangle {
            vertices: [v0, v1, v2],
        } = self.primitive;

        let PrimitiveStyle {
            fill_color,
            stroke_color,
            stroke_width,
            stroke_alignment,
            ..
        } = self.style;

        let (fill_color, stroke_color, stroke_alignment) =
            style_to_wasm!(fill_color, stroke_color, stroke_alignment);

        unsafe {
            ffi::widget::draw_triangle(
                v0.x,
                v0.y,
                v1.x,
                v1.y,
                v2.x,
                v2.y,
                fill_color,
                stroke_color,
                stroke_width,
                stroke_alignment,
            )
        }
    }
}
