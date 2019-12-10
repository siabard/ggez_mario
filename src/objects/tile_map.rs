use crate::objects::*;
use crate::reg::Reg;
use ggez::graphics;
use ggez::Context;

pub struct TileMap {
    kind: Vec<i32>,
    x: u32, // 가로 갯수
    y: u32, // 세로 갯수
    w: u32, // 타일 한 개의 너비
    h: u32, // 타일 한 개의 높이
}

impl TileMap {
    pub fn new(x: u32, y: u32) -> TileMap {
        let mut tile: Vec<i32> = vec![];
        for tx in 0..x {
            for ty in 0..y {
                tile.push(0);
            }
        }

        TileMap {
            kind: tile,
            x,
            y,
            w: 0,
            h: 0,
        }
    }

    pub fn set_wh(&mut self, (w, h): (u32, u32)) {
        self.w = w;
        self.h = h;
    }

    /// X / Y 번째의 타일을 변경한다.
    pub fn set_tile(&mut self, kind: i32, x: u32, y: u32) -> Result<(), &'static str> {
        let pos: usize = (y * self.x + x) as usize;
        match self.kind.get_mut(pos) {
            Some(elem) => {
                *elem = kind;
                Ok(())
            }
            None => Err("x,y is overflowed"),
        }
    }

    /// 전체 타일을 그린다.
    pub fn render(&self, ctx: &mut Context, reg: &mut Reg, ox: f32, oy: f32) {
        for ty in 0..self.y {
            for tx in 0..self.x {
                let pos: usize = (ty * self.x + tx) as usize;

                match self.kind.get(pos) {
                    Some(elem) => {
                        reg.draw_tile(
                            ctx,
                            *elem,
                            ox + (tx * self.w) as f32,
                            oy + (ty * self.h) as f32,
                        );
                        ()
                    }
                    None => (),
                }
            }
        }
    }

    /// 전체 타일 출력
    pub fn dump_tile(&self) {
        for ty in 0..self.y {
            for tx in 0..self.x {
                let pos: usize = (ty * self.x + tx) as usize;

                match self.kind.get(pos) {
                    Some(elem) => {
                        print!("{} ", *elem);
                    }
                    None => {
                        print!("x ");
                    }
                }
            }
            println!("");
        }
    }
}
