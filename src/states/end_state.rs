use crate::game;

use crate::objects::*;

use crate::reg::Reg;


use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;

use crate::states::*;

// 게임 종료화면
pub struct EndState {}

impl EndState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> EndState {
        let score = reg.get_i32("score".to_owned());
        let font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();
        let title = ggez::graphics::Text::new((format!("Your Score is {} ", score), font, 16.0));
        let start_menu = ggez::graphics::Text::new(("Push [Spage] To Return", font, 12.0));

        reg.add_font("font".to_owned(), font);
        reg.add_text("title".to_owned(), title);
        reg.add_text("start_menu".to_owned(), start_menu);

        init_global_sprite(reg);
        let state = EndState {};

        state
    }
}

// 메뉴 화면
impl States for EndState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, _dt: f32) -> StateResult {
        // 화살표를 눌러 상태를 변경한다.
        let pressed_key = ggez::input::keyboard::pressed_keys(ctx);

        if pressed_key.contains(&KeyCode::Space) {
            // reg 초기화
            reg.clear_font();
            reg.clear_image();
            reg.clear_sound();
            reg.clear_text();

            let init_state = InitState::new(ctx, reg);
            StateResult::Trans(Box::new(init_state))
        } else {
            StateResult::Void
        }
    }

    /// 모든 Render는 이제 자체에 포함된 buffer에만 그린다.
    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // 타이틀 (상단 5%, 각 메뉴 상단에서 85%, 95% 위치)
        let title = reg.get_text("title".to_owned()).unwrap();
        let start_menu = reg.get_text("start_menu".to_owned()).unwrap();
        let span = title.width(ctx) as f32;
        graphics::draw(
            ctx,
            title,
            (
                na::Point2::new(
                    (game::VIRTUAL_WIDTH - span) / 2.0,
                    game::VIRTUAL_HEIGHT * 0.05,
                ),
                0.0,
                ggez::graphics::WHITE,
            ),
        )
        .unwrap();

        let span = start_menu.width(ctx) as f32;
        graphics::draw(
            ctx,
            start_menu,
            (
                na::Point2::new(
                    (game::VIRTUAL_WIDTH - span) / 2.0,
                    game::VIRTUAL_HEIGHT * 0.85,
                ),
                0.0,
                ggez::graphics::Color::from_rgba(200, 200, 255, 255),
            ),
        )
        .unwrap();

        graphics::present(ctx).unwrap();

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}
