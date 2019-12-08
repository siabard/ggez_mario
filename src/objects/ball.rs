use crate::objects::*;

pub struct Ball {
    color: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Ball {
    pub fn new() -> Ball {
        // 화면 가운데에 위치시킨다.
        Ball {
            color: MAGENTA,
            x: game::VIRTUAL_WIDTH / 2.,
            y: game::VIRTUAL_HEIGHT - 40.,
            width: 8.,
            height: 8.,
            dx: 0.,
            dy: 0.,
        }
    }

    pub fn reset(&mut self) {
        self.y = game::VIRTUAL_HEIGHT - 40.;
        self.dx = 0.;
        self.dy = 0.;
    }

    pub fn fire(&mut self) {
        let mut rng = thread_rng();
        self.dx = rng.gen_range(-4, -2) as f32;
        self.dy = rng.gen_range(-4, -1) as f32;
    }
}

impl Object for Ball {
    fn update(&mut self, _ctx: &mut Context, reg: &mut Reg, _dt: f32) {
        if self.x < 0. || self.x > game::VIRTUAL_WIDTH {
            self.dx = -self.dx;
            //let sound = reg.get_sound_mut("wall-hit".to_owned()).unwrap();
            play_sound_once(&"wall-hit".to_owned(), reg);
        }
        if self.y < 0. {
            self.dy = -self.dy;
            //let sound = reg.get_sound_mut("wall-hit".to_owned()).unwrap();
            play_sound_once(&"wall-hit".to_owned(), reg);
        }

        self.x += self.dx;
        self.y += self.dy;
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        reg.draw_sprite(ctx, BALL_FLAG + self.color, self.x, self.y);
    }

    fn set_sprite(&mut self, idx: i32) {
        let color = idx & BALL_MASK;
        if color > 0 {
            self.color = color;
        }
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
