use anyhow::Result;
use cpu::Cpu;

mod cpu;

fn main() -> Result<()> {
    Cpu::new().load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00])
}
