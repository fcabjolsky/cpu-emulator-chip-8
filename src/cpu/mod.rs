use std::{panic, todo};

pub struct CPU {
    pub registers: [u8; 16],
    pub memory_position: usize,
    //todo: first 512bytes of memory are used for system
    pub memory: [u8; 0x1000],
    stack_pointer: usize,
    stack: [u16; 16],
}

impl CPU {
    pub fn new() -> Self {
        return CPU {
            registers: [0; 16],
            memory_position: 0,
            memory: [0; 0x1000],
            stack: [0; 16],
            stack_pointer: 0,
        };
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_op_code();
            self.memory_position += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let op_minor = ((opcode & 0x000F) >> 0) as u8;

            let addr = (opcode & 0x0FFF) as u16;
            let kk = (opcode & 0x00FF) as u8;

            match opcode {
                0x0000 => {
                    return;
                }
                0x00E0 => { /* CLEAR SCREEN */ }
                0x00EE => {
                    self.ret();
                }
                0x1000..=0x1FFF => {
                    self.jmp(addr);
                }
                0x2000..=0x2FFF => {
                    self.call(addr);
                }
                0x3000..=0x3FFF => {
                    self.se(x, kk);
                }
                0x4000..=0x4FFF => {
                    self.sne(x, kk);
                }
                0x5000..=0x5FFF => {
                    self.ser(x, y);
                }
                0x6000..=0x6FFF => {
                    self.ld(x, kk);
                }
                0x7000..=0x7FFF => {
                    self.add(x, kk);
                }
                0x8000..=0x8FFF => match op_minor {
                    0 => self.ld(x, self.registers[y as usize]),
                    1 => self.or_xy(x, y),
                    2 => self.and_xy(x, y),
                    3 => self.xor_xy(x, y),
                    4 => {
                        self.add_xy(x, y);
                    }
                    _ => {
                        todo!("opcode: {:04x}", opcode);
                    }
                },
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn read_op_code(&self) -> u16 {
        let op1 = self.memory[self.memory_position] as u16;
        let op2 = self.memory[self.memory_position + 1] as u16;
        return (op1 << 8) | op2;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        self.registers[0xF] = overflow as u8;
    }

    fn call(&mut self, mem_pos: u16) {
        if self.stack_pointer == self.stack.len() {
            panic!("Stack overflow");
        }
        self.stack[self.stack_pointer] = self.memory_position as u16;
        self.stack_pointer += 1;
        self.memory_position = mem_pos as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        self.stack_pointer -= 1;
        let previous_mem_position = self.stack[self.stack_pointer] as usize;
        self.memory_position = previous_mem_position;
    }

    fn jmp(&mut self, addr: u16) {
        self.memory_position = addr as usize;
    }

    fn se(&mut self, register: u8, nn: u8) {
        if self.registers[register as usize] == nn {
            self.memory_position += 2;
        }
    }

    fn sne(&mut self, register: u8, nn: u8) {
        if self.registers[register as usize] != nn {
            self.memory_position += 2;
        }
    }

    fn ser(&mut self, r1: u8, r2: u8) {
        if self.registers[r1 as usize] == self.registers[r2 as usize] {
            self.memory_position += 2;
        }
    }

    fn ld(&mut self, register: u8, nn: u8) {
        self.registers[register as usize] = nn;
    }

    fn add(&mut self, register: u8, nn: u8) {
        self.registers[register as usize] += nn;
    }

    fn or_xy(&mut self, r1: u8, r2: u8) {
        let r1_value = self.registers[r1 as usize];
        let r2_value = self.registers[r2 as usize];
        self.registers[r1 as usize] = r1_value | r2_value;
    }

    fn and_xy(&mut self, r1: u8, r2: u8) {
        let r1_value = self.registers[r1 as usize];
        let r2_value = self.registers[r2 as usize];
        self.registers[r1 as usize] = r1_value & r2_value;
    }

    fn xor_xy(&mut self, r1: u8, r2: u8) {
        let r1_value = self.registers[r1 as usize];
        let r2_value = self.registers[r2 as usize];
        self.registers[r1 as usize] = r1_value ^ r2_value;
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn add_three_registers_to_first_register() {
        let mut cpu = CPU::new();
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;

        let mem = &mut cpu.memory;
        mem[0] = 0x80;
        mem[1] = 0x14;
        mem[2] = 0x80;
        mem[3] = 0x24;
        mem[4] = 0x80;
        mem[5] = 0x34;

        cpu.run();

        assert_eq!(cpu.registers[0], 35);
    }

    #[test]
    fn complex_operation_using_functions() {
        let mut cpu = CPU {
            registers: [0; 16],
            memory_position: 0,
            memory: [0; 0x1000],
            stack: [0; 16],
            stack_pointer: 0,
        };

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x21;
        mem[0x001] = 0x00; //call
        mem[0x002] = 0x21;
        mem[0x003] = 0x00; //call
        mem[0x004] = 0x00;
        mem[0x005] = 0x00;

        //function: add r1 to r0 twice
        mem[0x100] = 0x80;
        mem[0x101] = 0x14;
        mem[0x102] = 0x80;
        mem[0x103] = 0x14;
        mem[0x104] = 0x00;
        mem[0x105] = 0xEE;

        cpu.run();

        assert_eq!(cpu.registers[0], 45);
    }

    #[test]
    #[should_panic(expected = "Stack overflow")]
    fn stack_overflow() {
        let mut cpu = CPU::new();

        let mem = &mut cpu.memory;
        mem[0x000] = 0x20;
        mem[0x001] = 0x02; //call
        mem[0x002] = 0x20;
        mem[0x003] = 0x04; //call
        mem[0x004] = 0x20;
        mem[0x005] = 0x06; //call
        mem[0x006] = 0x20;
        mem[0x007] = 0x08; //call
        mem[0x008] = 0x20;
        mem[0x009] = 0x0a; //call
        mem[0x00a] = 0x20;
        mem[0x00b] = 0x0c; //call
        mem[0x00c] = 0x20;
        mem[0x00d] = 0x0e; //call
        mem[0x00e] = 0x20;
        mem[0x00f] = 0x10; //call
        mem[0x010] = 0x20;
        mem[0x011] = 0x12; //call
        mem[0x012] = 0x20;
        mem[0x013] = 0x14; //call
        mem[0x014] = 0x20;
        mem[0x015] = 0x16; //call
        mem[0x016] = 0x20;
        mem[0x017] = 0x18; //call
        mem[0x018] = 0x20;
        mem[0x019] = 0x1a; //call
        mem[0x01a] = 0x20;
        mem[0x01b] = 0x1c; //call
        mem[0x01c] = 0x20;
        mem[0x01d] = 0x1e; //call
        mem[0x01e] = 0x20;
        mem[0x01f] = 0x20; //call
        mem[0x020] = 0x20;
        mem[0x021] = 0x22; //call

        cpu.run();
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn stack_underflow() {
        let mut cpu = CPU::new();

        let mem = &mut cpu.memory;
        mem[0x000] = 0x00;
        mem[0x001] = 0xEE;

        cpu.run();
    }

    #[test]
    fn after_jump_memory_position_is_correct() {
        let mut cpu = CPU::new();

        let mem = &mut cpu.memory;
        mem[0x000] = 0x12;
        mem[0x001] = 0x22;

        cpu.run();
        // 0x222 + 2 because of the last run
        assert_eq!(cpu.memory_position, 0x224);
    }

    #[test]
    fn skip_comparing_register_with_number() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 5;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x30;
        mem[0x001] = 0x05;

        cpu.run();
        // 0x004 + 2 because of the last run
        assert_eq!(cpu.memory_position, 0x006);
    }

    #[test]
    fn skip_comparing_two_registers() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 5;
        cpu.registers[1] = 5;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x50;
        mem[0x001] = 0x10;

        cpu.run();
        // 0x004 + 2 because of the last run
        assert_eq!(cpu.memory_position, 0x006);
    }

    #[test]
    fn skip_comparing_two_registers_not_equal() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x40;
        mem[0x001] = 0x10;

        cpu.run();
        // 0x004 + 2 because of the last run
        assert_eq!(cpu.memory_position, 0x006);
    }

    #[test]
    fn load_to_register() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 5;
        cpu.registers[1] = 5;

        assert_eq!(cpu.registers[0], 5);
        let mem = &mut cpu.memory;
        mem[0x000] = 0x60;
        mem[0x001] = 0x0A;

        cpu.run();
        assert_eq!(cpu.registers[0], 10);
        assert_eq!(cpu.registers[1], 5);
    }

    #[test]
    fn load_register1_to_register0() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 5;
        cpu.registers[1] = 15;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x80;
        mem[0x001] = 0x10;

        cpu.run();
        assert_eq!(cpu.registers[0], 15);
        assert_eq!(cpu.registers[1], 15);
    }

    #[test]
    fn add_without_carry_flag() {
        let mut cpu = CPU::new();

        cpu.registers[0] = 5;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x70;
        mem[0x001] = 0x0A;

        cpu.run();
        assert_eq!(cpu.registers[0], 15);
    }
}
