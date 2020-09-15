use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Default)]
pub struct KeyboardState {
  pressed_keys: [bool; 0x10],
}

impl KeyboardState {
  pub fn handle_winit_input(&mut self, input: &WinitInputHelper) {
    self.update_key_state(input, VirtualKeyCode::Key1);
    self.update_key_state(input, VirtualKeyCode::Key2);
    self.update_key_state(input, VirtualKeyCode::Key3);
    self.update_key_state(input, VirtualKeyCode::Key4);
    self.update_key_state(input, VirtualKeyCode::Q);
    self.update_key_state(input, VirtualKeyCode::W);
    self.update_key_state(input, VirtualKeyCode::E);
    self.update_key_state(input, VirtualKeyCode::R);
    self.update_key_state(input, VirtualKeyCode::A);
    self.update_key_state(input, VirtualKeyCode::S);
    self.update_key_state(input, VirtualKeyCode::D);
    self.update_key_state(input, VirtualKeyCode::F);
    self.update_key_state(input, VirtualKeyCode::Z);
    self.update_key_state(input, VirtualKeyCode::X);
    self.update_key_state(input, VirtualKeyCode::C);
    self.update_key_state(input, VirtualKeyCode::V);
  }

  fn update_key_state(&mut self, input: &WinitInputHelper, key_code: VirtualKeyCode) {
    let index = KeyboardState::key_code_to_index(key_code);
    if input.key_pressed(key_code) {
      self.pressed_keys[index] = true;
    } else if input.key_released(key_code) {
      self.pressed_keys[index] = false;
    }
  }

  fn key_code_to_index(key_code: VirtualKeyCode) -> usize {
    match key_code {
      VirtualKeyCode::Key1 => 0x1,
      VirtualKeyCode::Key2 => 0x2,
      VirtualKeyCode::Key3 => 0x3,
      VirtualKeyCode::Key4 => 0xC,
      VirtualKeyCode::Q => 0x4,
      VirtualKeyCode::W => 0x5,
      VirtualKeyCode::E => 0x6,
      VirtualKeyCode::R => 0xD,
      VirtualKeyCode::A => 0x7,
      VirtualKeyCode::S => 0x8,
      VirtualKeyCode::D => 0x9,
      VirtualKeyCode::F => 0xE,
      VirtualKeyCode::Z => 0xA,
      VirtualKeyCode::X => 0x0,
      VirtualKeyCode::C => 0xB,
      VirtualKeyCode::V => 0xF,
      _ => panic!("Unsupported key: {:?}", key_code),
    }
  }

  pub fn is_key_pressed(&self, key_index: u8) -> bool {
    return self.pressed_keys[key_index as usize];
  }

  pub fn get_pressed_keys(&self) -> Vec<u8> {
    let mut keys_pressed: Vec<u8> = vec![];
    for (i, pressed) in self.pressed_keys.iter().enumerate() {
      if *pressed {
        keys_pressed.push(i as u8);
      }
    }
    keys_pressed
  }
}
