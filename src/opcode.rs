#[derive(Debug)]
pub enum Opcode {
  Nop,
  Cls,
  Ret,
  Jp { addr: u16 },
  Call { addr: u16 },
  Se { x: u8, byte: u8 },
  Sne { x: u8, byte: u8 },
  LdImm { x: u8, byte: u8 },
  AddImm { x: u8, byte: u8 },
  AddR { x: u8, y: u8 },
  SubR { x: u8, y: u8},
  LdR { x: u8, y: u8 },
  LdiImm { addr: u16 },
  Drw { x: u8, y: u8, size: u8 },
  Sknp { x: u8 },
  Skp { x: u8 },
  LdRK { x: u8 },
  AddiR { x: u8 },
  LdM { x: u8 },
  SetDt { x: u8 },
  SetSt { x: u8 },
  LdDt { x: u8 },
  And { x: u8, y: u8 },
  Shr { x: u8 },
  XorR { x: u8, y: u8 },
  Rnd { x: u8, byte: u8 }
}

impl Opcode {
  fn read_nnn(value: u16) -> u16 {
    value & 0x0FFF
  }

  fn read_n(value: u16) -> u8 {
    (value & 0x000F) as u8
  }

  fn read_x(value: u16) -> u8 {
    ((value >> 8) & 0b1111) as u8
  }

  fn read_y(value: u16) -> u8 {
    ((value >> 4) & 0b1111) as u8
  }

  fn read_kk(value: u16) -> u8 {
    (value & 0x00FF) as u8
  }

  pub fn decode(instruction: u16) -> Self {
    let nibbles = (
      (instruction & 0xF000) >> 12 as u8,
      (instruction & 0x0F00) >> 8 as u8,
      (instruction & 0x00F0) >> 4 as u8,
      (instruction & 0x000F) as u8,
    );

    match nibbles {
      (0x00, 0x00, 0x0E, 0x00) => Opcode::Cls,
      (0x00, 0x00, 0x0E, 0x0E) => Opcode::Ret,
      (0x01, _, _, _) => Opcode::Jp {
        addr: Opcode::read_nnn(instruction),
      },
      (0x02, _, _, _) => Opcode::Call {
        addr: Opcode::read_nnn(instruction),
      },
      (0x03, _, _, _) => Opcode::Se {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x04, _, _, _) => Opcode::Sne {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x06, _, _, _) => Opcode::LdImm {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x07, _, _, _) => Opcode::AddImm {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x08, _, _, 0x00) => Opcode::LdR {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x02) => Opcode::And {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x3) => Opcode::XorR {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x4) => Opcode::AddR {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x5) => Opcode::SubR {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x06) => Opcode::Shr {
        x: Opcode::read_x(instruction),
      },
      (0x0A, _, _, _) => Opcode::LdiImm {
        addr: Opcode::read_nnn(instruction),
      },
      (0x0C, _, _, _) => Opcode::Rnd {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction)
      },
      (0x0D, _, _, _) => Opcode::Drw {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
        size: Opcode::read_n(instruction),
      },
      (0x0E, _, 0x9, 0xE) => Opcode::Skp {
        x: Opcode::read_x(instruction),
      },
      (0x0E, _, 0x0A, 0x01) => Opcode::Sknp {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x00, 0x07) => Opcode::LdDt {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x00, 0x0A) => Opcode::LdRK {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x01, 0x05) => Opcode::SetDt {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x01, 0x08) => Opcode::SetSt {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x01, 0x0E) => Opcode::AddiR {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x06, 0x05) => Opcode::LdM {
        x: Opcode::read_x(instruction),
      },
      _ => Opcode::Nop, // panic!("Unrecognized instruction {:#X?}", instruction),
    }
  }
}
