use crate::game;
use crate::level_maker;

use crate::objects::{self, Ball, Block, Object, Paddle};
use crate::reg::Reg;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;

use crate::states::*;

#[derive(PartialEq)]
pub enum PlayStateMode {
    READY,
    GO,
}

pub struct PlayState {
    paused: bool,
    paddle: Paddle,
    ball: Ball,
    blocks: Vec<Block>,
    score: i32,
    health: i32,
    level: i32,
    mode: PlayStateMode,
}

impl PlayState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> PlayState {
        let default_font = ggez::graphics::Font::new(ctx, "/font.ttf").unwrap();

        reg.add_font("default".to_owned(), default_font);

        let paddle = Paddle::new();

        let ball = Ball::new();

        // 배경 음악
        reg.add_sound(
            "music".to_owned(),
            audio::Source::new(ctx, "/music.wav").unwrap(),
        );

        // 효과음
        reg.add_sound(
            "paddle-hit".to_owned(),
            audio::Source::new(ctx, "/paddle_hit.wav").unwrap(),
        );

        reg.add_sound(
            "score".to_owned(),
            audio::Source::new(ctx, "/score.wav").unwrap(),
        );

        reg.add_sound(
            "wall-hit".to_owned(),
            audio::Source::new(ctx, "/wall_hit.wav").unwrap(),
        );

        reg.add_sound(
            "brick-hit-1".to_owned(),
            audio::Source::new(ctx, "/brick-hit-1.wav").unwrap(),
        );

        reg.add_sound(
            "brick-hit-2".to_owned(),
            audio::Source::new(ctx, "/brick-hit-2.wav").unwrap(),
        );

        reg.add_sound(
            "hurt".to_owned(),
            audio::Source::new(ctx, "/hurt.wav").unwrap(),
        );

        reg.add_sound(
            "victory".to_owned(),
            audio::Source::new(ctx, "/victory.wav").unwrap(),
        );
        reg.add_sound(
            "recover".to_owned(),
            audio::Source::new(ctx, "/recover.wav").unwrap(),
        );
        reg.add_sound(
            "high-score".to_owned(),
            audio::Source::new(ctx, "/high_score.wav").unwrap(),
        );
        reg.add_sound(
            "pause".to_owned(),
            audio::Source::new(ctx, "/pause.wav").unwrap(),
        );

        play_bgm(&"music".to_owned(), reg);

        // 블럭 초기화하기
        let blocks = level_maker::create_map(1);

        // score, health, level 값 가져오기
        reg.add_i32("score".to_owned(), 0);
        reg.add_i32("health".to_owned(), 3);

        PlayState {
            paused: false,
            paddle,
            ball,
            blocks,
            health: 3,
            level: 1,
            score: 0,
            mode: PlayStateMode::READY,
        }
    }
}
impl States for PlayState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) -> StateResult {
        // 나중에 지울거임..
        // 키 입력할 때 크기 바꿀라구..
        let color: i32 = if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key1) {
            objects::BLUE
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key2) {
            objects::RED
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key3) {
            objects::GREEN
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key4) {
            objects::MAGENTA
        } else {
            0
        };

        // 나중에 지울꺼임..
        // 키 입력할 때 색상 바꿀라구..
        let size: i32 = if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key5) {
            objects::SMALL
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key6) {
            objects::MEDIUM
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key7) {
            objects::LARGE
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Key8) {
            objects::HUGE
        } else {
            0
        };

        if self.mode == PlayStateMode::READY
            && ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Space)
        {
            self.mode = PlayStateMode::GO;
            self.ball.fire();
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::X) {
            reg.clear_font();
            StateResult::PopState
        } else {
            if self.paused == false {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.paused = true;

                    //let music = reg.get_sound_mut("music".to_owned()).unwrap();
                    stop_sound(&("music".to_owned()), reg);

                    //let sound = reg.get_sound_mut("pause".to_owned()).unwrap();
                    play_sound_once(&("pause".to_owned()), reg);
                }

                // paddle 처리
                if color != 0 || size != 0 {
                    self.paddle.set_sprite(objects::PADDLE_FLAG + color + size);
                }
                self.paddle.update(ctx, reg, dt);

                // 공처리
                self.ball.update(ctx, reg, dt);

                // 게임 상태가 READY이면 공은 paddle을 따라다녀야한다.
                if self.mode == PlayStateMode::READY {
                    self.ball.x = self.paddle.x + self.paddle.width / 2.;
                }

                if self.ball.y > game::VIRTUAL_HEIGHT {
                    // 죽음..
                    self.health = self.health - 1;

                    if self.health <= 0 {
                        reg.clear_font();
                        reg.clear_image();
                        reg.clear_sound();
                        reg.clear_text();

                        let end_state = EndState::new(ctx, reg);

                        StateResult::Trans(Box::new(end_state))
                    } else {
                        self.mode = PlayStateMode::READY;
                        self.ball.reset();
                        StateResult::Void
                    }
                } else {
                    // 두 물체의 충돌처리
                    let collide = objects::collide_aabb(&self.paddle, &self.ball);
                    if collide.contains(&CollideFlag::TOP) {
                        self.ball.dy = -self.ball.dy;
                        //let sound = reg.get_sound_mut("paddle-hit".to_owned()).unwrap();
                        play_sound_once(&("paddle-hit".to_owned()), reg);
                    }

                    // 블럭하고 충돌처리
                    for block in self.blocks.iter_mut() {
                        if block.inplay == true {
                            let collide = objects::collide_aabb(&self.ball, block);
                            if collide.len() > 0 {
                                block.hit(reg);

                                // 공 상단 / 하단
                                if collide.contains(&CollideFlag::TOP) && self.ball.dy < 0.
                                    || collide.contains(&CollideFlag::BOTTOM) && self.ball.dy > 0.
                                {
                                    self.ball.dy = -self.ball.dy;
                                }
                                // 공 좌측 / 우측
                                if collide.contains(&CollideFlag::LEFT) && self.ball.dx < 0.
                                    || collide.contains(&CollideFlag::RIGHT) && self.ball.dx > 0.
                                {
                                    self.ball.dx = -self.ball.dx;
                                }
                            }
                        }
                    }

                    StateResult::Void
                }
            } else {
                if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Return) {
                    self.paused = false;
                    //let music = reg.get_sound_mut("music".to_owned()).unwrap();
                    //music.set_repeat(true);
                    stop_sound(&("music".to_owned()), reg);
                }

                StateResult::Void
            }
        }
    }

    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for block in self.blocks.iter_mut() {
            block.draw(ctx, reg);
        }

        self.paddle.draw(ctx, reg);

        self.ball.draw(ctx, reg);
        if self.paused == true {
            let message = ggez::graphics::Text::new((
                "Game Paused\n\nPress [Enter] To Resume",
                *reg.get_font("default".to_owned()).unwrap(),
                16.0,
            ));

            let span = message.width(ctx) as f32;
            graphics::draw(
                ctx,
                &message,
                (
                    na::Point2::new(
                        (game::VIRTUAL_WIDTH - span) / 2.0,
                        game::VIRTUAL_HEIGHT / 2.0,
                    ),
                    0.0,
                    graphics::WHITE,
                ),
            )
            .unwrap();
        }

        // 생명 출력하기
        let health = self.health;

        let mut hx = 0.;

        for _ in 0..health {
            reg.draw_heart(ctx, objects::HEARTS_FLAG, hx, 0.);
            hx = hx + 11.;
        }

        for _ in health..3 {
            reg.draw_heart(ctx, objects::HEARTS_FLAG + 1, hx, 0.);
            hx = hx + 11.;
        }

        graphics::present(ctx).unwrap();

        graphics::set_canvas(ctx, None);

        StateResult::Void
    }
}
