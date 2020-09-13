#[derive(Debug)]
pub enum Opcode {
  NOP,
  CLS,
  RET,
  JP { addr: u16 },
  CALL { addr: u16 },
  SE { x: u8, byte: u8 },
  SNE { x: u8, byte: u8 },
  LD_IMM { x: u8, byte: u8 },
  ADD_IMM { x: u8, byte: u8 },
  ADD_R { x: u8, y: u8 },
  SUB_R { x: u8, y: u8},
  LD_R { x: u8, y: u8 },
  LDI_IMM { addr: u16 },
  DRW { x: u8, y: u8, size: u8 },
  SKNP { x: u8 },
  SKP { x: u8 },
  LD_R_K { x: u8 },
  ADDI_R { x: u8 },
  LD_M { x: u8 },
  SET_DT { x: u8 },
  SET_ST { x: u8 },
  LD_DT { x: u8 },
  AND { x: u8, y: u8 },
  SHR { x: u8 },
  XOR_R { x: u8, y: u8 },
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
      (0x00, 0x00, 0x0E, 0x00) => Opcode::CLS,
      (0x00, 0x00, 0x0E, 0x0E) => Opcode::RET,
      (0x01, _, _, _) => Opcode::JP {
        addr: Opcode::read_nnn(instruction),
      },
      (0x02, _, _, _) => Opcode::CALL {
        addr: Opcode::read_nnn(instruction),
      },
      (0x03, _, _, _) => Opcode::SE {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x04, _, _, _) => Opcode::SNE {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x06, _, _, _) => Opcode::LD_IMM {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x07, _, _, _) => Opcode::ADD_IMM {
        x: Opcode::read_x(instruction),
        byte: Opcode::read_kk(instruction),
      },
      (0x08, _, _, 0x00) => Opcode::LD_R {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x02) => Opcode::AND {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x3) => Opcode::XOR_R {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x4) => Opcode::ADD_R {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x5) => Opcode::SUB_R {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
      },
      (0x08, _, _, 0x06) => Opcode::SHR {
        x: Opcode::read_x(instruction),
      },
      (0x0A, _, _, _) => Opcode::LDI_IMM {
        addr: Opcode::read_nnn(instruction),
      },
      (0x0D, _, _, _) => Opcode::DRW {
        x: Opcode::read_x(instruction),
        y: Opcode::read_y(instruction),
        size: Opcode::read_n(instruction),
      },
      (0x0E, _, 0x9, 0xE) => Opcode::SKP {
        x: Opcode::read_x(instruction),
      },
      (0x0E, _, 0x0A, 0x01) => Opcode::SKNP {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x00, 0x07) => Opcode::LD_DT {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x00, 0x0A) => Opcode::LD_R_K {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x01, 0x05) => Opcode::SET_DT {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x01, 0x08) => Opcode::SET_ST {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x01, 0x0E) => Opcode::ADDI_R {
        x: Opcode::read_x(instruction),
      },
      (0x0F, _, 0x06, 0x05) => Opcode::LD_M {
        x: Opcode::read_x(instruction),
      },
      _ => Opcode::NOP, // panic!("Unrecognized instruction {:#X?}", instruction),
    }
  }
}
