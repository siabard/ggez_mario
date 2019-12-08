use crate::objects::*;

pub struct Paddle {
    size: i32,
    color: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
}

impl Paddle {
    fn get_width(&self) -> f32 {
        match self.size {
            SMALL => 32.,
            MEDIUM => 64.,
            LARGE => 96.,
            HUGE => 128.,
            _ => 32.,
        }
    }

    pub fn new() -> Paddle {
        // 화면 가운데에 위치시킨다.
        Paddle {
            size: MEDIUM,
            color: MAGENTA,
            x: game::VIRTUAL_WIDTH / 2.,
            y: game::VIRTUAL_HEIGHT - 32.,
            width: 64.,
            height: 16.,
            dx: 0.,
        }
    }
}

impl Object for Paddle {
    fn update(&mut self, ctx: &mut Context, _reg: &mut Reg, dt: f32) {
        if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Left) {
            self.dx = -1. * PADDLE_SPEED;
        } else if ggez::input::keyboard::is_key_pressed(ctx, ggez::input::keyboard::KeyCode::Right)
        {
            self.dx = PADDLE_SPEED;
        } else {
            self.dx = 0.;
        }

        if self.dx < 0. {
            self.x = (self.x + self.dx * dt).max(0.);
        } else if self.dx > 0. {
            self.x = game::VIRTUAL_WIDTH.min(self.x + self.dx * dt);
        }
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        reg.draw_sprite(ctx, PADDLE_FLAG + self.color + self.size, self.x, self.y);
    }

    fn set_sprite(&mut self, idx: i32) {
        let color = idx & COLOR_MASK;
        if color > 0 {
            self.color = color;
        }
        let size = idx & SIZE_MASK;
        if size > 0 {
            self.size = size;
        }
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
