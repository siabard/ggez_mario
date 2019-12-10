use crate::objects::*;
use crate::reg::Reg;

pub struct Tile {
    kind: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

impl Tile {
    pub fn new(kind: i32) -> Tile {
        Tile {
            kind,
            width: 0.,
            height: 0.,
            x: 0.,
            y: 0.,
            dx: 0.,
            dy: 0.,
        }
    }
}

impl Object for Tile {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) {
        ()
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        ()
    }

    fn set_sprite(&mut self, idx: i32) {
        self.kind = idx;
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
