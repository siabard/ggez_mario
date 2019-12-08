//! states는 다양한 게임내 State를 정의한다.
//! GameState : 게임 진행 상태
//! InitState : 초기 시작 상태
//! MenuState : 메뉴 상태



use crate::objects::*;

use crate::reg::Reg;

use ggez::audio::SoundSource;
use ggez::graphics::{Canvas};


use ggez::Context;

pub mod end_state;
pub mod init_state;
pub mod pause_state;
pub mod play_state;

pub use end_state::EndState;
pub use init_state::InitState;
pub use pause_state::PauseState;
pub use play_state::PlayState;

pub enum StateResult {
    PushState(Box<dyn States>),
    PopState,
    Trans(Box<dyn States>),
    Void,
}

pub trait States {
    fn update(&mut self, ctx: &mut Context, reg: &mut Reg, dt: f32) -> StateResult;
    fn render(&mut self, ctx: &mut Context, reg: &mut Reg, buffer: &mut Canvas) -> StateResult;
}

pub fn play_sound_once(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    if sound.playing() == false {
        sound.set_repeat(false);
        sound.play().unwrap();
    }
}

pub fn play_sound(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    if sound.playing() == false {
        sound.play().unwrap();
    }
}

pub fn play_bgm(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    sound.set_repeat(true);
    if sound.playing() == false {
        sound.play().unwrap();
    }
}

pub fn stop_sound(name: &String, reg: &mut Reg) {
    let sound = reg.get_sound_mut((*name).clone()).unwrap();
    if sound.playing() == true {
        sound.stop();
    }
}
