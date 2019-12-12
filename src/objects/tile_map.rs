use crate::objects::*;
use crate::reg::Reg;
use ggez::graphics;
use ggez::Context;

pub struct TileMap {
    kind: Vec<i32>,
    x: u32,  // 가로 갯수
    y: u32,  // 세로 갯수
    w: u32,  // 타일 한 개의 너비
    h: u32,  // 타일 한 개의 높이
    ox: f32, // 가상의 화면에서 해당 타일이 노출될 때 시작할 x 기점
    oy: f32, // 가상의 화면에서 해당 타일이 노출될 때 시작할 y 기점
}

impl TileMap {
    pub fn new(x: u32, y: u32, ox: f32, oy: f32) -> TileMap {
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
            ox,
            oy,
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
    /// 다만 타일을 그리더라도,
    /// Clipping 영역을 벗어나면 못그린다.
    /// cx, cy, cw, ch -> 카메라의 x,y,w,h 영역
    pub fn render(&self, ctx: &mut Context, reg: &mut Reg, cx: f32, cy: f32, cw: f32, ch: f32) {
        for ty in 0..self.y {
            for tx in 0..self.x {
                let pos: usize = (ty * self.x + tx) as usize;

                match self.kind.get(pos) {
                    Some(elem) => {
                        let sx: f32 = self.ox + (tx * self.w) as f32; // 원 기점에서 해당 타일을 그리기 시작할 X 위치
                        let sy: f32 = self.oy + (ty * self.h) as f32; // 윈 기점에서 해당 타일을 그리기 시작할 Y 위치

                        // 카메라 영역에 타일이 노출될 수 있으면 (일종의 Collision q이라생)
                        // 출력한다.
                        if (sx < cx + cw)
                            && (sx + self.w as f32 > cx)
                            && (sy < cy + ch)
                            && (sy + self.h as f32 > cy)
                        {
                            reg.draw_tile(ctx, *elem, sx - cx, sy - cy);
                        } else {
                        }
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
