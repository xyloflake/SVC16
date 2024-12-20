use crate::RES;
use anyhow::Result;
use pixels::Pixels;
use rodio::Source;
use std::hash::Hash;
use winit::{
    event::MouseButton,
    event_loop::EventLoopWindowTarget,
    keyboard::{Key, KeyCode},
};
use winit_input_helper::WinitInputHelper;
use winit_input_map::{input_map, GamepadButton, InputMap};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum NesInput {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

pub fn build_gamepad_map() -> InputMap<NesInput> {
    input_map!(
        (NesInput::A, GamepadButton::East),
        (NesInput::B, GamepadButton::South),
        (NesInput::Select, GamepadButton::Select),
        (NesInput::Start, GamepadButton::Start),
        (NesInput::Up, GamepadButton::DPadUp),
        (NesInput::Down, GamepadButton::DPadDown),
        (NesInput::Left, GamepadButton::DPadLeft),
        (NesInput::Right, GamepadButton::DPadRight)
    )
}

pub fn read_u16s_from_file(file_path: &str) -> Result<Vec<u16>> {
    use std::io::{BufReader, Read};
    let file = std::fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 2];
    let mut u16s = Vec::new();
    while reader.read_exact(&mut buffer).is_ok() {
        let value = u16::from_le_bytes(buffer);
        u16s.push(value);
    }
    Ok(u16s)
}

fn rgb565_to_argb(rgb565: u16) -> (u8, u8, u8) {
    let r = ((rgb565 >> 11) & 0x1F) as u8;
    let g = ((rgb565 >> 5) & 0x3F) as u8;
    let b = (rgb565 & 0x1F) as u8;
    let r = (r << 3) | (r >> 2);
    let g = (g << 2) | (g >> 4);
    let b = (b << 3) | (b >> 2);
    (r, g, b)
}

pub fn update_image_buffer(imbuff: &mut [u8], screen: &[u16; RES * RES]) {
    for i in 0..RES * RES {
        let col = rgb565_to_argb(screen[i]);
        *imbuff.get_mut(4 * i).expect("Error with image buffer") = col.0;
        *imbuff.get_mut(4 * i + 1).expect("Error with image buffer") = col.1;
        *imbuff.get_mut(4 * i + 2).expect("Error with image buffer") = col.2;
        *imbuff.get_mut(4 * i + 3).expect("Error with image buffer") = 255;
    }
}

pub fn get_input_code(
    input: &WinitInputHelper,
    gamepad: &InputMap<NesInput>,
    pxls: &Pixels,
) -> (u16, u16) {
    let raw_mp = input.cursor().unwrap_or((0., 0.));
    let mp = match pxls.window_pos_to_pixel(raw_mp) {
        Ok(p) => p,
        Err(ev) => pxls.clamp_pixel_pos(ev),
    };
    let pos_code = (mp.1 as u16 * 256) + mp.0 as u16;
    let mut key_code = 0_u16;
    if input.key_held(KeyCode::Space)
        || input.mouse_held(MouseButton::Left)
        || gamepad.pressing(NesInput::A)
    {
        key_code += 1;
    }
    if input.key_held_logical(Key::Character("b"))
        || input.mouse_held(MouseButton::Right)
        || gamepad.pressing(NesInput::B)
    {
        key_code += 2;
    }
    if input.key_held_logical(Key::Character("w"))
        || input.key_held(KeyCode::ArrowUp)
        || gamepad.pressing(NesInput::Up)
    {
        key_code += 4;
    }
    if input.key_held_logical(Key::Character("s"))
        || input.key_held(KeyCode::ArrowDown)
        || gamepad.pressing(NesInput::Down)
    {
        key_code += 8;
    }
    if input.key_held_logical(Key::Character("a"))
        || input.key_held(KeyCode::ArrowLeft)
        || gamepad.pressing(NesInput::Left)
    {
        key_code += 16;
    }
    if input.key_held_logical(Key::Character("d"))
        || input.key_held(KeyCode::ArrowRight)
        || gamepad.pressing(NesInput::Right)
    {
        key_code += 32;
    }
    if input.key_held_logical(Key::Character("n")) || gamepad.pressing(NesInput::Select) {
        key_code += 64;
    }
    if input.key_held_logical(Key::Character("m")) || gamepad.pressing(NesInput::Start) {
        key_code += 128;
    }
    (pos_code, key_code)
}

pub fn handle_event_loop_error(handle: &EventLoopWindowTarget<()>, msg: impl AsRef<str>) {
    eprintln!("{}", msg.as_ref());
    handle.exit();
}

pub struct SoundFormat {
    samples: Vec<u16>,
    current: usize,
}

impl SoundFormat {
    pub fn new(vec: Vec<u16>) -> Self {
        Self {
            samples: vec,
            current: 0,
        }
    }
}
impl Iterator for SoundFormat {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self
            .samples
            .get(self.current)
            .map(|v| (*v as f32 - 32768.0) / 32768.0);
        self.current += 1;
        val
    }
}
impl Source for SoundFormat {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        16000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
