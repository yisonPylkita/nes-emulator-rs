use std::ops::Add;

use anyhow::{anyhow, Result};

enum AddressingMode {
    // TODO: Check these
    Immediate,
    Absolute,
}

pub struct Cpu {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: u8,
    pub program_counter: u16,
    pub memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: 0,
            reg_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn execute(&mut self, code: Vec<u8>) -> Result<()> {
        if code.len() > u16::MAX as usize {
            return Err(anyhow!("Code to execute is too big"));
        }
        self.program_counter = 0;
        loop {
            if self.program_counter >= code.len() as u16 {
                break;
            }
            let opcode = code[self.program_counter as usize];
            // TODO: what about PC overflow?
            self.program_counter += 1;
            match opcode {
                0xa9 => {
                    self.reg_a = code[self.program_counter as usize];
                    self.program_counter += 1;
                    self.status = generate_new_status(self.status, self.reg_a);
                }
                0xaa => {
                    self.reg_x = self.reg_a;
                    self.status = generate_new_status(self.status, self.reg_x);
                }
                0xe8 => {
                    self.reg_x = self.reg_x.wrapping_add(1);
                    self.status = generate_new_status(self.status, self.reg_x);
                }
                _ => todo!(),
            }
        }
        Ok(())
    }

    fn mem_read(&self, index: u16) -> u8 {
        self.memory[index as usize]
    }

    fn mem_read_u16(&self, index: u16) -> u16 {
        let low = self.mem_read(index) as u16;
        let high = self.mem_read(index + 1) as u16;
        (high << 8) | low
    }

    fn mem_write(&mut self, index: u16, value: u8) {
        self.memory[index as usize] = value;
    }
    fn mem_write_u16(&mut self, index: u16, value: u16) {
        let value_low = (value >> 8) as u8;
        let value_high = (value & 0xFF) as u8;
        self.mem_write(index, value_low);
        self.mem_write(index + 1, value_high);
    }

    fn get_operand_address(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute => self.program_counter,
            AddressingMode::Immediate => self.mem_read_u16(self.program_counter),
        }
    }

    fn lda(&mut self, mode: AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.mem_read(address);
        self.reg_a = value;
    }
}

fn generate_new_status(old_status: u8, result: u8) -> u8 {
    let mut status = old_status;
    if result == 0 {
        status |= 0b0000_0010;
    } else {
        status &= 0b1111_1101;
    }
    if result & 0b1000_0000 != 0 {
        status |= 0b1000_0000;
    } else {
        status &= 0b0111_1111;
    }

    status
}

#[cfg(test)]
mod test {
    use super::*;
    use assert::assert_ok;
    use pretty_assertions::assert_eq;

    #[test]
    fn cpu_after_init_should_have_all_registers_clear() {
        let cpu = Cpu::new();
        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.status, 0);
        assert_eq!(cpu.program_counter, 0);
    }

    #[test]
    fn execute_lda_with_zero() {
        let mut cpu = Cpu::new();
        assert_ok!(cpu.execute(vec![0xa9, 0x00]));

        // TODO: name status bits properly
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.status, 0x02);
        assert_eq!(cpu.program_counter, 2);
    }

    #[test]
    fn execute_lda_with_value_with_bit_7_set() {
        let mut cpu = Cpu::new();
        assert_ok!(cpu.execute(vec![0xa9, 0x80]));

        // TODO: name status bits properly
        assert_eq!(cpu.reg_a, 0x80);
        assert_eq!(cpu.status, 0x80);
        assert_eq!(cpu.program_counter, 2);
    }

    #[test]
    fn execute_tax_for_a_clean_cpu() {
        let mut cpu = Cpu::new();
        assert_ok!(cpu.execute(vec![0xaa]));

        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.status, 0x02);
        assert_eq!(cpu.program_counter, 1);
    }

    #[test]
    fn execute_tax_for_reg_a_between_1_and_7f() {
        for reg_a in 0x01..=0x7f {
            let mut cpu = Cpu::new();
            cpu.reg_a = reg_a;
            assert_ok!(cpu.execute(vec![0xaa]));

            assert_eq!(cpu.reg_a, reg_a);
            assert_eq!(cpu.reg_x, reg_a);
            assert_eq!(cpu.status, 0x00);
            assert_eq!(cpu.program_counter, 1);
        }
    }

    #[test]
    fn execute_tax_for_reg_a_between_80_and_ff() {
        for reg_a in 0x80..=0xff {
            let mut cpu = Cpu::new();
            cpu.reg_a = reg_a;
            assert_ok!(cpu.execute(vec![0xaa]));

            assert_eq!(cpu.reg_a, reg_a);
            assert_eq!(cpu.reg_x, reg_a);
            assert_eq!(cpu.status, 0x80);
            assert_eq!(cpu.program_counter, 1);
        }
    }

    #[test]
    fn execute_inx_for_reg_x_between_0_and_7e() {
        for reg_x in 0x00..=0x7e {
            let mut cpu = Cpu::new();
            cpu.reg_x = reg_x;
            assert_ok!(cpu.execute(vec![0xe8]));

            assert_eq!(cpu.reg_a, 0);
            assert_eq!(cpu.reg_x, reg_x + 1);
            assert_eq!(cpu.status, 0x00);
            assert_eq!(cpu.program_counter, 1);
        }
    }

    #[test]
    fn execute_inx_for_reg_x_between_7f_and_fe() {
        for reg_x in 0x7f..=0xfe {
            let mut cpu = Cpu::new();
            cpu.reg_x = reg_x;
            assert_ok!(cpu.execute(vec![0xe8]));

            assert_eq!(cpu.reg_a, 0);
            assert_eq!(cpu.reg_x, reg_x + 1);
            assert_eq!(cpu.status, 0x80);
            assert_eq!(cpu.program_counter, 1);
        }
    }

    #[test]
    fn execute_inx_for_reg_x_being_ff() {
        let mut cpu = Cpu::new();
        cpu.reg_x = 0xff;
        assert_ok!(cpu.execute(vec![0xe8]));

        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.reg_x, 0);
        assert_eq!(cpu.status, 0x02);
        assert_eq!(cpu.program_counter, 1);
    }

    #[test]
    fn status_tests() {
        assert_eq!(generate_new_status(0x00, 0x00), 0x02);
        assert_eq!(generate_new_status(0x00, 0x80), 0x80);
        assert_eq!(generate_new_status(0x80, 0x00), 0x02);
        assert_eq!(generate_new_status(0x82, 0x00), 0x02);
    }
}
