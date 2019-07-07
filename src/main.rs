use std::io;
use std::result::Result::Ok;

use invaders_emulator::architecture::cpu::CpuContext;
use invaders_emulator::disassembler;

fn main() -> io::Result<()> {
    let program = "resources/invaders.rom";

    disassembler::disassemble(program)?;

    let mut cpu = CpuContext::load_program(program, 10000);
    cpu.run();

    Ok(())
}