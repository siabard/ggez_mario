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
        }
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
        println!("{}", sprite);
        reg.draw_sprite(
            ctx,
            *(self.frames.get(self.current_frame).unwrap()),
            self.x,
            self.y,
        );
    }

    fn set_sprite(&mut self, _idx: i32) {
        ()
    }

    fn get_xywh(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}
