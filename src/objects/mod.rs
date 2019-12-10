use crate::game;
use crate::states::{play_sound, play_sound_once};

use crate::reg::Reg;
use ggez::Context;
use rand::*;

pub const PADDLE_FLAG: i32 = 0b0000_0000_0000_0000_0001_0000_0000_0000;
pub const BALL_FLAG: i32 = 0b0000_0000_0000_0000_0010_0000_0000_0000;
pub const BLOCK_FLAG: i32 = 0b0000_0000_0000_0000_0100_0000_0000_0000;
pub const HEARTS_FLAG: i32 = 0b0000_0000_0000_0000_1000_0000_0000_0000;
pub const BLUE: i32 = 1;
pub const GREEN: i32 = 2;
pub const RED: i32 = 4;
pub const MAGENTA: i32 = 8;
pub const STAT_1: i32 = 0b0001_0000;
pub const STAT_2: i32 = 0b0010_0000;
pub const STAT_3: i32 = 0b0100_0000;
pub const COLOR_MASK: i32 = 0b1111;
pub const BALL_MASK: i32 = 0b1111_1111;

pub const SMALL: i32 = 0b0001_0000;
pub const MEDIUM: i32 = 0b0010_0000;
pub const LARGE: i32 = 0b0100_0000;
pub const HUGE: i32 = 0b1000_0000;
pub const SIZE_MASK: i32 = 0b1111_0000;

pub const PADDLE_SPEED: f32 = 200.;

pub mod ball;
pub mod block;
pub mod paddle;
pub mod tile;
pub mod tile_map;

pub use crate::objects::ball::Ball;
pub use crate::objects::block::Block;
pub use crate::objects::paddle::Paddle;
pub use crate::objects::tile::Tile;
pub use crate::objects::tile_map::TileMap;

#[derive(PartialEq, Debug)]
pub enum CollideFlag {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    NONE,
}

pub trait Object {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32);
    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg);
    fn set_sprite(&mut self, idx: i32);
    fn get_xywh(&self) -> (f32, f32, f32, f32);
}

/// AABB Collide
pub fn collide_aabb(a: &dyn Object, b: &dyn Object) -> Vec<CollideFlag> {
    let a_xywh = a.get_xywh();
    let b_xywh = b.get_xywh();

    if (a_xywh.0 < b_xywh.0 + b_xywh.2)
        && (a_xywh.0 + a_xywh.2 > b_xywh.0)
        && (a_xywh.1 < b_xywh.1 + b_xywh.3)
        && (a_xywh.1 + a_xywh.3 > b_xywh.1)
    {
        // 충돌시에 어느 면에 부딪히는지를 알려준다.
        // a.x < b.x -> LEFT
        // a.x > b.x -> RIGHT
        // a.y < b.y -> BOTTOM
        // a.y > b.y -> TOP
        let mut vec = Vec::<CollideFlag>::new();
        if a_xywh.0 < b_xywh.0 {
            vec.push(CollideFlag::LEFT);
        }

        if a_xywh.0 > b_xywh.0 {
            vec.push(CollideFlag::RIGHT);
        }

        if a_xywh.1 < b_xywh.1 {
            vec.push(CollideFlag::BOTTOM);
        }

        if a_xywh.1 > b_xywh.1 {
            vec.push(CollideFlag::TOP);
        }

        vec
    } else {
        vec![]
    }
}

// 기본적인 스프라이트 데이터를 모두 초기화한다.
pub fn init_global_sprite(reg: &mut Reg) {
    // Paddle
    reg.register_sprite(PADDLE_FLAG + BLUE + SMALL, 0., 64., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + BLUE + MEDIUM, 32., 64., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + BLUE + LARGE, 96., 64., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + BLUE + HUGE, 0., 80., 128., 16.);

    reg.register_sprite(PADDLE_FLAG + GREEN + SMALL, 0., 96., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + GREEN + MEDIUM, 32., 96., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + GREEN + LARGE, 96., 96., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + GREEN + HUGE, 0., 112., 128., 16.);

    reg.register_sprite(PADDLE_FLAG + RED + SMALL, 0., 128., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + RED + MEDIUM, 32., 128., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + RED + LARGE, 96., 128., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + RED + HUGE, 0., 144., 128., 16.);

    reg.register_sprite(PADDLE_FLAG + MAGENTA + SMALL, 0., 160., 32., 16.);
    reg.register_sprite(PADDLE_FLAG + MAGENTA + MEDIUM, 32., 160., 64., 16.);
    reg.register_sprite(PADDLE_FLAG + MAGENTA + LARGE, 96., 160., 96., 16.);
    reg.register_sprite(PADDLE_FLAG + MAGENTA + HUGE, 0., 176., 128., 16.);

    // Ball

    reg.register_sprite(BALL_FLAG + BLUE, 96., 48., 8., 8.);
    reg.register_sprite(BALL_FLAG + GREEN, 104., 48., 8., 8.);
    reg.register_sprite(BALL_FLAG + RED, 112., 48., 8., 8.);
    reg.register_sprite(BALL_FLAG + MAGENTA, 120., 48., 8., 8.);

    reg.register_sprite(BALL_FLAG + STAT_1, 96., 56., 8., 8.);
    reg.register_sprite(BALL_FLAG + STAT_2, 104., 56., 8., 8.);
    reg.register_sprite(BALL_FLAG + STAT_3, 112., 56., 8., 8.);

    // block
    // 블럭 종류는 총 21개임
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;

    for i in 1..22 {
        reg.register_sprite(BLOCK_FLAG + i, x, y, 32., 16.);

        if i % 6 == 0 {
            x = 0.;
            y = y + 16.;
        } else {
            x = x + 32.;
        }
    }

    // hearts
    reg.register_heart(HEARTS_FLAG, 0., 0., 10., 9.);
    reg.register_heart(HEARTS_FLAG + 1, 10., 0., 10., 9.);
}

pub fn init_tiles(reg: &mut Reg) {
    reg.register_tile(1, 0., 0., 32., 32.);
    reg.register_tile(2, 32., 0., 32., 32.);
}
