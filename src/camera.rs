//! 카메라 클래스
//! 해당 클래스에서 정의된 영역에 대해서만 화면에 출력한다.
//! 일종의 스크린 버퍼임
use ggez;
use ggez::Context;

pub struct Camera {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    pub buffer: ggez::graphics::Canvas,
}

impl Camera {
    pub fn new(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) -> Self {
        let buffer =
            ggez::graphics::Canvas::new(ctx, w as u16, h as u16, ggez::conf::NumSamples::One)
                .unwrap();

        Camera { x, y, w, h, buffer }
    }
}
