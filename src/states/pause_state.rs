



use crate::reg::Reg;


use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::KeyCode;

use ggez::Context;

use crate::states::*;

pub struct PauseState {}

impl PauseState {
    pub fn new() -> PauseState {
        PauseState {}
    }
}

impl States for PauseState {
    fn update(&mut self, ctx: &mut Context, _reg: &mut Reg, _dt: f32) -> StateResult {
        // X가 눌러지면 스테이트 종료
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            StateResult::PopState
        } else {
            StateResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context, _reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        graphics::present(ctx).unwrap();

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}
