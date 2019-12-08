use crate::objects::*;

pub struct Block {
    pub color: i32,
    pub tier: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub inplay: bool,
}

impl Block {
    pub fn new(ox: f32, oy: f32) -> Block {
        // Block 설치하기
        Block {
            color: 1,
            tier: 0,
            x: ox,
            y: oy,
            width: 32.,
            height: 16.,
            dx: 0.,
            dy: 0.,
            inplay: true,
        }
    }

    pub fn hit(&mut self, reg: &mut Reg) {
        self.inplay = false;
        play_sound(&"brick-hit-2".to_owned(), reg);
    }
}

impl Object for Block {
    fn update(&mut self, _ctx: &mut Context, _reg: &mut Reg, _dt: f32) {
        ()
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        if self.inplay {
            reg.draw_sprite(
                ctx,
                BLOCK_FLAG + 1 + (self.color - 1) * 4 + self.tier,
                self.x,
                self.y,
            );
        }
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
