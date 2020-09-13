use minifb::{Key, KeyRepeat, Window};

#[derive(Debug, Default)]
pub struct KeyboardState {
  pressed_keys: [bool; 0x10],
}

impl KeyboardState {
  pub fn get_keyoard_state(window: &mut Window) -> Self {
    window.update();
    let mut keyboard_state = KeyboardState::default();
    let keys = window.get_keys_pressed(KeyRepeat::Yes).unwrap();
    for key in keys.iter() {
      match key {
        Key::Key1 => keyboard_state.pressed_keys[0x1] = true,
        Key::Key2 => keyboard_state.pressed_keys[0x2] = true,
        Key::Key3 => keyboard_state.pressed_keys[0x3] = true,
        Key::Key4 => keyboard_state.pressed_keys[0xC] = true,
        Key::Q => keyboard_state.pressed_keys[0x4] = true,
        Key::W => keyboard_state.pressed_keys[0x5] = true,
        Key::E => keyboard_state.pressed_keys[0x6] = true,
        Key::R => keyboard_state.pressed_keys[0xD] = true,
        Key::A => keyboard_state.pressed_keys[0x7] = true,
        Key::S => keyboard_state.pressed_keys[0x8] = true,
        Key::D => keyboard_state.pressed_keys[0x9] = true,
        Key::F => keyboard_state.pressed_keys[0xE] = true,
        Key::Z => keyboard_state.pressed_keys[0xA] = true,
        Key::X => keyboard_state.pressed_keys[0x0] = true,
        Key::C => keyboard_state.pressed_keys[0xB] = true,
        Key::V => keyboard_state.pressed_keys[0xF] = true,
        _ => {}
      }
    }

    keyboard_state
  }

  pub fn is_key_pressed(&self, key_index: u8) -> bool {
    return self.pressed_keys[key_index as usize];
  }
}
