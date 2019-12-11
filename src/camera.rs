//! 카메라 클래스
//! 해당 클래스에서 정의된 영역에 대해서만 화면에 출력한다.
//! 출력이 필요한 항목을 제한하게된다.
use ggez;
use ggez::Context;

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub ox: f32,
    pub oy: f32,
    pub buffer: ggez::graphics::Canvas,
}

impl Camera {
    pub fn new(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, ox: f32, oy: f32) -> Self {
        let buffer =
            ggez::graphics::Canvas::new(ctx, w as u16, h as u16, ggez::conf::NumSamples::One)
                .unwrap();

        Camera {
            x,
            y,
            w,
            h,
            ox,
            oy,
            buffer,
        }
    }
}
