use anyhow::{anyhow, Result};

pub struct Cpu {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: u8,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: 0,
            reg_x: 0,
            status: 0,
            pc: 0,
        }
    }

    pub fn execute(&mut self, code: Vec<u8>) -> Result<()> {
        if code.len() > u16::MAX as usize {
            return Err(anyhow!("Code to execute is too big"));
        }
        self.pc = 0;
        loop {
            if self.pc >= code.len() as u16 {
                break;
            }
            let opcode = code[self.pc as usize];
            self.pc += 1;
            match opcode {
                0xa9 => {
                    self.reg_a = code[self.pc as usize];
                    self.pc += 1;
                    if self.reg_a == 0x00 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= 0b1111_1101;
                    }
                    if self.reg_a & 0b1000_0000 != 0 {
                        self.status |= 0b1000_0000;
                    } else {
                        self.status &= 0b0111_1111;
                    }
                }
                0xaa => {
                    self.reg_x = self.reg_a;
                    if self.reg_a == 0 {
                        self.status |= 0b0000_0010;
                    } else {
                        self.status &= 0b1111_1101;
                    }
                    if self.reg_a & 0b1000_0000 != 0 {
                        self.status |= 0b1000_0000;
                    } else {
                        self.status &= 0b0111_1111;
                    }
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
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
        assert_eq!(cpu.pc, 0);
    }

    #[test]
    fn execute_lda_with_zero() {
        let mut cpu = Cpu::new();
        assert_ok!(cpu.execute(vec![0xa9, 0x00]));

        // TODO: name status bits properly
        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.status, 0x02);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn execute_lda_with_value_with_bit_7_set() {
        let mut cpu = Cpu::new();
        assert_ok!(cpu.execute(vec![0xa9, 0x80]));

        // TODO: name status bits properly
        assert_eq!(cpu.reg_a, 0x80);
        assert_eq!(cpu.status, 0x80);
        assert_eq!(cpu.pc, 2);
    }

    #[test]
    fn execute_tax_for_a_clean_cpu() {
        let mut cpu = Cpu::new();
        assert_ok!(cpu.execute(vec![0xaa]));

        assert_eq!(cpu.reg_a, 0x00);
        assert_eq!(cpu.reg_x, 0x00);
        assert_eq!(cpu.status, 0x02);
        assert_eq!(cpu.pc, 1);
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
            assert_eq!(cpu.pc, 1);
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
            assert_eq!(cpu.pc, 1);
        }
    }
}
