use super::display;
use super::memory;

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

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        cpu.reg_pc = 0x200;

        cpu
    }

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

    pub fn step(&mut self, memory: &mut memory::Memory, display: &mut display::Display) {
        let instruction = memory.read_doublebyte(self.reg_pc);
        let nibbles = (
            (instruction & 0xF000) >> 12 as u8,
            (instruction & 0x0F00) >> 8 as u8,
            (instruction & 0x00F0) >> 4 as u8,
            (instruction & 0x000F) as u8,
        );

        let program_counter = match nibbles {
            (0x00, 0x00, 0x0E, 0x0E) => self.ret(),
            (0x01, _, _, _) => self.jp_addr(Cpu::read_nnn(instruction)),
            (0x02, _, _, _) => self.call_addr(Cpu::read_nnn(instruction)),
            (0x03, _, _, _) => self.se_vx_byte(Cpu::read_x(instruction), Cpu::read_kk(instruction)),
            (0x06, _, _, _) => self.ld_vx_byte(Cpu::read_x(instruction), Cpu::read_kk(instruction)),
            (0x07, _, _, _) => {
                self.add_vx_byte(Cpu::read_x(instruction), Cpu::read_kk(instruction))
            }
            (0x08, _, _, 0x00) => self.ld_vx_vy(Cpu::read_x(instruction), Cpu::read_y(instruction)),
            (0x0A, _, _, _) => self.ld_i_addr(Cpu::read_nnn(instruction)),
            (0x0D, _, _, _) => self.drw_vx_vy_nibble(
                Cpu::read_x(instruction),
                Cpu::read_y(instruction),
                Cpu::read_n(instruction),
                memory,
                display,
            ),
            (0x0E, _, 0x0A, 0x01) => self.sknp_vx(Cpu::read_x(instruction)),
            (0x0F, _, 0x01, 0x0E) => self.add_i_vx(Cpu::read_x(instruction)),
            (0x0F, _, 0x06, 0x05) => self.ld_vx_i(Cpu::read_x(instruction), memory),
            _ => panic!("Unrecognized instruction {:#x?} {:#x?}", instruction, self),
        };

        match program_counter {
            ProgramCounter::Next => self.reg_pc += 2,
            ProgramCounter::Jump(addr) => self.reg_pc = addr,
            ProgramCounter::Skip => self.reg_pc += 4,
        };

        std::thread::sleep(std::time::Duration::from_millis(10));
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
            return ProgramCounter::Skip;
        }

        ProgramCounter::Next
    }

    fn ld_vx_byte(&mut self, x: u8, byte: u8) -> ProgramCounter {
        self.reg_gp[x as usize] = byte;
        ProgramCounter::Next
    }

    fn add_vx_byte(&mut self, x: u8, byte: u8) -> ProgramCounter {
        self.reg_gp[x as usize] += byte;
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

    fn sknp_vx(&mut self, x: u8) -> ProgramCounter {
        let pressed = false;
        if pressed == false {
            return ProgramCounter::Skip;
        }

        ProgramCounter::Next
    }

    fn ld_vx_i(&mut self, x: u8, memory: &mut memory::Memory) -> ProgramCounter {
        for i in 0..x + 1 {
            self.reg_gp[i as usize] = memory.read_byte(self.reg_i + (i as u16));
        }

        ProgramCounter::Next
    }

    fn ret(&mut self) -> ProgramCounter {
        let addr = self.stack[self.reg_sp as usize - 1];
        self.reg_sp -= 1;

        ProgramCounter::Jump(addr)
    }
}

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}
