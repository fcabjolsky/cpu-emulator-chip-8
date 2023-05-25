use std::todo;

pub struct CPU {
    pub registers: [u8; 16],
    pub memory_position: usize,
    //todo: first 512bytes of memory are used for system
    pub memory: [u8; 0x1000],
}

impl CPU {
    pub fn run(&mut self) {
        loop {
            let opcode = self.read_op_code();
            self.memory_position += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {return;}
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("todo"),
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
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn add_three_registers_to_first_register() {
        let mut cpu = CPU {
            registers: [0; 16], 
            memory_position: 0,
            memory: [0; 0x1000],
        };

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;
        
        let mem = &mut cpu.memory;
        mem[0] = 0x80; mem[1] = 0x14;
        mem[2] = 0x80; mem[3] = 0x24;
        mem[4] = 0x80; mem[5] = 0x34;

        cpu.run();

        assert_eq!(cpu.registers[0], 35);
    }
}
