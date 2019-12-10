use crate::game;

use crate::objects::*;

use crate::reg::Reg;
use ggez::audio;

use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;

use crate::states::*;

pub struct InitState {
    tile_map: TileMap,
}

impl InitState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> InitState {
        init_tiles(reg);
        let mut tile_map = TileMap::new(5, 5);

        tile_map.set_tile(1, 0, 0);
        tile_map.set_tile(1, 1, 0);
        tile_map.set_tile(1, 2, 0);
        tile_map.set_tile(1, 3, 0);
        tile_map.set_tile(1, 4, 0);

        tile_map.set_tile(2, 0, 1);
        //tile_map.set_tile(2, 1, 1);
        //tile_map.set_tile(2, 2, 1);
        //tile_map.set_tile(2, 3, 1);
        //tile_map.set_tile(2, 4, 1);

        tile_map.set_tile(1, 0, 2);
        tile_map.set_tile(1, 1, 2);
        tile_map.set_tile(1, 2, 2);
        tile_map.set_tile(1, 3, 2);
        tile_map.set_tile(1, 4, 2);

        tile_map.set_tile(2, 0, 3);
        //tile_map.set_tile(2, 1, 3);
        //tile_map.set_tile(2, 2, 3);
        //tile_map.set_tile(2, 3, 3);
        //tile_map.set_tile(2, 4, 3);

        tile_map.set_wh((32, 32));

        let state = InitState { tile_map };

        state
    }
}

// 메뉴 화면
impl States for InitState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, _dt: f32) -> StateResult {
        StateResult::Void
    }

    /// 모든 Render는 이제 자체에 포함된 buffer에만 그린다.
    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.tile_map.render(ctx, reg, 0., 0.);

        graphics::present(ctx).unwrap();

        ggez::graphics::set_canvas(ctx, None);
        StateResult::Void
    }
}
