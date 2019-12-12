use crate::game;

use crate::objects::*;

use crate::camera::Camera;
use crate::reg::Reg;
use ggez::audio;

use ggez::graphics::{self, Canvas};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::Context;

use crate::states::*;

/// 이제 각 Statet에서는 내용을 출력할 Camera를 지정해야한다.
pub struct InitState {
    tile_map: TileMap,
    player: Player,
    camera: Camera,
}

impl InitState {
    pub fn new(ctx: &mut Context, reg: &mut Reg) -> InitState {
        init_tiles(reg);
        let mut tile_map = TileMap::new(5, 5, 0., 0.);

        // 메인 카메라는 화면 크기만큼 출력한다.
        let camera = Camera::new(
            ctx,
            0.,
            0.,
            game::VIRTUAL_WIDTH,
            game::VIRTUAL_HEIGHT,
            0.,
            0.,
        );

        let player = Player::new();

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

        crate::objects::init_player(reg);
        let state = InitState {
            tile_map,
            player,
            camera,
        };

        state
    }
}

// 메뉴 화면
impl States for InitState {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) -> StateResult {
        // 캔버스를 가로로 한픽셀씩 옮긴다.
        self.camera.x = self.camera.x + 1.;
        self.player.update(ctx, reg, dt);
        StateResult::Void
    }

    /// 모든 Render는 이제 자체에 포함된 camera에 그린다.
    /// 이때 camera의 영역에 포함되어야만 그려야한다.

    fn render(&mut self, ctx: &mut Context, reg: &mut Reg) -> StateResult {
        ggez::graphics::set_canvas(ctx, Some(&self.camera.buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.].into());

        self.tile_map.render(
            ctx,
            reg,
            self.camera.x,
            self.camera.y,
            self.camera.w,
            self.camera.h,
        );

        self.player.draw(ctx, reg);
        graphics::present(ctx).unwrap();

        ggez::graphics::set_canvas(ctx, None);

        // canvas buffer를 윈도우에 출력
        //
        let dest_point = na::Point2::new(0., 0.);
        graphics::draw(
            ctx,
            &self.camera.buffer,
            graphics::DrawParam::new()
                .dest(dest_point)
                .src(graphics::Rect::new(0., 0., 1., 1.)),
        )
        .unwrap();

        StateResult::Void
    }
}
