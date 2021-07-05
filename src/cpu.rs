pub struct Cpu {
    pub reg_a: u8,
    pub status: u8,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_a: 0,
            status: 0,
            pc: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cpu_after_init_should_have_all_registers_clear() {
        let cpu = Cpu::new();
        assert_eq!(cpu.reg_a, 0);
        assert_eq!(cpu.status, 0);
        assert_eq!(cpu.pc, 0);
    }
}
