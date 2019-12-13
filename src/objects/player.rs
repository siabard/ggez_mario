use crate::camera::Camera;
use crate::objects::*;
use ggez;

pub struct Player {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub frames: Vec<i32>,
    pub current_frame: usize,
    pub timer: f32,
    pub anim_span: f32,
    pub ox: f32,
    pub oy: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            width: 16.,
            height: 16.,
            x: 0.,
            y: 0.,
            dx: 0.,
            dy: 0.,
            frames: vec![
                PADDLE_FLAG + 0,
                PADDLE_FLAG + 1,
                PADDLE_FLAG + 2,
                PADDLE_FLAG + 3,
            ],
            timer: 0.,
            current_frame: 0,
            anim_span: 2.,
            ox: 0.,
            oy: 0.,
        }
    }

    pub fn transform(&mut self, camera: &mut Camera) {
        // camera 좌표를 토대로 ox / oy를 설정한다.
        // ox, oy : 화면에 drawingl되는 위치
        let mut cx = camera.x;
        let mut cy = camera.y;

        // x, y가 폭의 10% 내외영역이면
        // 카메라의 x, y를 안전한 거리만큼 옮겨야한다.
        println!(
            "camera x: {}, player x: {}, span : {}",
            cx,
            self.x,
            (camera.w * 0.1)
        );
        if self.x <= cx + camera.w * 0.1 {
            cx = cx - (cx + camera.w * 0.1 - self.x);

            if cx < 0. {
                cx = 0.
            }
        } else if self.x >= cx + camera.w * 0.9 {
            cx = self.x - camera.w * 0.9;

            if cx > 1000. - camera.w {
                cx = 1000. - camera.w;
            }
        }

        if self.y <= cy + camera.h * 0.1 {
            cy = cy - (cy + camera.h * 0.1 - self.y);

            if cy < 0. {
                cy = 0.
            }
        } else if self.y >= cy + camera.h * 0.9 {
            cy = self.y - camera.h * 0.9;

            if cy > 1000. - camera.h {
                cy = 1000. - camera.h;
            }
        }

        camera.x = cx;
        camera.y = cy;

        self.ox = self.x - cx;
        self.oy = self.y - cy;
    }
}

impl Animate for Player {
    fn animate(&mut self, dt: f32) {
        // dt를 자체 타이머에 더하고
        // anim_span 보다 크면 0으로 리셋한다.
        self.timer = self.timer + dt;

        if self.timer >= self.anim_span {
            self.timer = 0.;
        }

        // frames 의 길이만큼 anim_span 구간내에서 돌아야하니까
        // anim_span과 timer 구간 비례만큼 frames를 바꾼다.

        let len = self.frames.len();
        self.current_frame = (self.timer * (len as f32) / self.anim_span) as usize;
    }
}

impl Object for Player {
    fn update(&mut self, ctx: &mut Context, _reg: &mut Reg, dt: f32) {
        self.animate(dt);
    }

    fn draw(&mut self, ctx: &mut Context, reg: &mut Reg) {
        let sprite = *(self.frames.get(self.current_frame).unwrap());
        reg.draw_sprite(
            ctx,
            *(self.frames.get(self.current_frame).unwrap()),
            self.ox,
            self.oy,
        );
    }

    fn set_sprite(&mut self, _idx: i32) {
        ()
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
