use super::display;
use super::keyboard::KeyboardState;
use super::memory;
use super::opcode::Opcode;

#[derive(Debug, Default)]
pub struct Cpu {
    reg_gp: [u8; 16],
    reg_i: u16,
    reg_sound_timer: u8,
    reg_delay: u8,
    reg_pc: u16,
    reg_sp: u8,
    stack: [u16; 16],
}

pub struct ProgramChange {
    pub redraw: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        cpu.reg_pc = 0x200;

        cpu
    }

    pub fn tick_timers(&mut self) {
        if self.reg_delay > 0 {
            self.reg_delay -= 1;
        }

        if self.reg_sound_timer > 0 {
            self.reg_sound_timer -= 1;
        }
    }

    pub fn step(
        &mut self,
        memory: &mut memory::Memory,
        display: &mut display::Display,
        keyboard_state: &KeyboardState,
    ) -> ProgramChange {
        let instruction = memory.read_doublebyte(self.reg_pc);
        let opcode = Opcode::decode(instruction);
        let mut program_change = ProgramChange { redraw: false };

        println!("{:#X?} - {:#X?}: {:x?}", self.reg_pc, instruction, opcode);
        // pause();
        let program_counter = match opcode {
            Opcode::CALL { addr } => {
                program_change.redraw = true;
                self.call_addr(addr)
            }
            Opcode::CLS => self.cls(display),
            Opcode::RET => self.ret(),
            Opcode::JP { addr } => self.jp_addr(addr),
            Opcode::SE { x, byte } => self.se_vx_byte(x, byte),
            Opcode::SNE { x, byte } => self.sne_vx_byte(x, byte),
            Opcode::LD_IMM { x, byte } => self.ld_vx_byte(x, byte),
            Opcode::ADD_IMM { x, byte } => self.add_vx_byte(x, byte),
            Opcode::ADD_R { x, y } => self.add_vx_vy(x, y),
            Opcode::SUB_R { x, y } => self.sub_vx_vy(x,y),
            Opcode::LD_R { x, y } => self.ld_vx_vy(x, y),
            Opcode::LDI_IMM { addr } => self.ld_i_addr(addr),
            Opcode::DRW { x, y, size } => {
                program_change.redraw = true;
                self.drw_vx_vy_nibble(x, y, size, memory, display)
            }
            Opcode::SKNP { x } => self.sknp_vx(x, keyboard_state),
            Opcode::SKP { x } => self.skp_vx(x, keyboard_state),
            Opcode::ADDI_R { x } => self.add_i_vx(x),
            Opcode::LD_M { x } => self.ld_vx_i(x, memory),
            Opcode::SET_DT { x } => self.set_dt(x),
            Opcode::SET_ST { x } => self.set_st(x),
            Opcode::LD_DT { x } => self.ld_dt(x),
            Opcode::AND { x, y } => self.and(x, y),
            Opcode::SHR { x } => self.shr(x),
            Opcode::XOR_R { x, y } => self.xor_vx_vy(x, y),
            Opcode::LD_R_K { x } => self.ld_vx_k(x, keyboard_state),
            _ => panic!("Instruction: {:#X?} - not handled", instruction),
        };

        match program_counter {
            ProgramCounter::Next => self.reg_pc += 2,
            ProgramCounter::Jump(addr) => self.reg_pc = addr,
            ProgramCounter::Skip => self.reg_pc += 4,
            ProgramCounter::Wait => {}
        };

        // println!("I: {:#X?} GP: {:X?}", self.reg_i, self.reg_gp);

        program_change
    }

    fn jp_addr(&mut self, addr: u16) -> ProgramCounter {
        ProgramCounter::Jump(addr)
    }

    fn call_addr(&mut self, addr: u16) -> ProgramCounter {
        self.stack[self.reg_sp as usize] = self.reg_pc;
        self.reg_sp += 1;
        ProgramCounter::Jump(addr)
    }
    // skip equal
    fn se_vx_byte(&mut self, x: u8, byte: u8) -> ProgramCounter {
        if self.reg_gp[x as usize] == byte {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    fn sne_vx_byte(&mut self, x: u8, byte: u8) -> ProgramCounter {
        if self.reg_gp[x as usize] == byte {
            ProgramCounter::Next
        } else {
            ProgramCounter::Skip
        }
    }

    fn ld_vx_byte(&mut self, x: u8, byte: u8) -> ProgramCounter {
        self.reg_gp[x as usize] = byte;
        ProgramCounter::Next
    }

    fn add_vx_byte(&mut self, x: u8, byte: u8) -> ProgramCounter {
        let val: u16 = (self.reg_gp[x as usize] as u16) + (byte as u16);
        self.reg_gp[x as usize] = (val & 0xFF) as u8;
        ProgramCounter::Next
    }

    fn add_vx_vy(&mut self, x: u8, y: u8) -> ProgramCounter {
        let val: u16 = (self.reg_gp[x as usize] as u16) + (self.reg_gp[y as usize] as u16);
        self.reg_gp[0xF] = if val > 0xFF { 1 } else { 0 };
        self.reg_gp[x as usize] = (val & 0xFF) as u8;
        ProgramCounter::Next
    }

    fn sub_vx_vy(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.reg_gp[x as usize] = if self.reg_gp[x as usize] > self.reg_gp[y as usize] {
            self.reg_gp[0xF] = 1;
            self.reg_gp[x as usize] - self.reg_gp[y as usize]
        } else {
            self.reg_gp[0xF] = 0;
            0
        };

        ProgramCounter::Next
    }

    fn ld_i_addr(&mut self, addr: u16) -> ProgramCounter {
        self.reg_i = addr;
        ProgramCounter::Next
    }

    fn drw_vx_vy_nibble(
        &mut self,
        x: u8,
        y: u8,
        nibble: u8,
        memory: &mut memory::Memory,
        display: &mut display::Display,
    ) -> ProgramCounter {
        let sprite = memory.read_chunk(self.reg_i, nibble as usize);
        let mut set_vflag = false;
        display.draw(
            self.reg_gp[x as usize],
            self.reg_gp[y as usize],
            sprite,
            &mut set_vflag,
        );

        self.reg_gp[0xF] = set_vflag as u8;

        ProgramCounter::Next
    }

    fn add_i_vx(&mut self, x: u8) -> ProgramCounter {
        self.reg_i += self.reg_gp[x as usize] as u16;
        ProgramCounter::Next
    }

    fn ld_vx_vy(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.reg_gp[x as usize] = self.reg_gp[y as usize];
        ProgramCounter::Next
    }

    fn sknp_vx(&mut self, x: u8, keyboard_state: &KeyboardState) -> ProgramCounter {
        if keyboard_state.is_key_pressed(self.reg_gp[x as usize]) {
            ProgramCounter::Next
        } else {
            ProgramCounter::Skip
        }
    }

    fn skp_vx(&mut self, x: u8, keyboard_state: &KeyboardState) -> ProgramCounter {
        if keyboard_state.is_key_pressed(self.reg_gp[x as usize]) {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }

    fn ld_vx_i(&mut self, x: u8, memory: &mut memory::Memory) -> ProgramCounter {
        for i in 0..x + 1 {
            self.reg_gp[i as usize] = memory.read_byte(self.reg_i + (i as u16));
        }

        ProgramCounter::Next
    }

    fn set_dt(&mut self, x: u8) -> ProgramCounter {
        self.reg_delay = self.reg_gp[x as usize];
        ProgramCounter::Next
    }

    fn set_st(&mut self, x: u8) -> ProgramCounter {
        self.reg_sound_timer = self.reg_gp[x as usize];
        ProgramCounter::Next
    }

    fn ld_dt(&mut self, x: u8) -> ProgramCounter {
        self.reg_gp[x as usize] = self.reg_delay;
        ProgramCounter::Next
    }

    fn ret(&mut self) -> ProgramCounter {
        let addr = self.stack[self.reg_sp as usize - 1];
        self.reg_sp -= 1;

        ProgramCounter::Jump(addr + 2)
    }

    fn cls(&mut self, display: &mut display::Display) -> ProgramCounter {
        display.clear();
        ProgramCounter::Next
    }

    fn and(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.reg_gp[x as usize] &= self.reg_gp[y as usize];
        ProgramCounter::Next
    }

    fn shr(&mut self, x: u8) -> ProgramCounter {
        self.reg_gp[0xF] = self.reg_gp[x as usize] & 0b1;
        self.reg_gp[x as usize] /= 2;
        ProgramCounter::Next
    }

    fn xor_vx_vy(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.reg_gp[x as usize] ^= self.reg_gp[y as usize];
        ProgramCounter::Next
    }

    fn ld_vx_k(&mut self, x: u8, keyboard_state: &KeyboardState) -> ProgramCounter {
        let pressed_keys = keyboard_state.get_pressed_keys();
        if pressed_keys.len() > 0 {
            self.reg_gp[x as usize] = pressed_keys[0];
            ProgramCounter::Next
        } else {
            ProgramCounter::Wait
        }
    }
}

enum ProgramCounter {
    Wait,
    Next,
    Skip,
    Jump(u16),
}
