fn get_opcode(instruction: u32) -> u32 {
    return instruction & 0b1111111;
}

fn get_rd(instruction: u32) -> usize {
    return ((instruction >> 7) & 0b11111) as usize;
}

fn get_funct3(instruction: u32) -> u32 {
    return (instruction >> 12) & 0b111;
}

fn get_rs1(instruction: u32) -> usize {
    return ((instruction >> 15) & 0b11111) as usize;
}

fn get_imm(instruction: u32) -> u32 {
    return instruction >> 20; // = (instruction >> 20) & 0b111111111111
}

fn set_register(index: usize, registers: &mut [u32; 32], value: u32) {
    match index {
        0 => {}
        1..32 => registers[index] = value,
        _ => panic!("not exists register")
    }
}

fn sign_extend_12(unsigned_12: u32) -> u32 {
    return (((unsigned_12 << 20) as i32) >> 20) as u32
}

fn addi(rd: usize, rs1: usize, imm: u32, registers: &mut [u32; 32]) {
    println!(
        "[ADDI] rd: {:#X}, rs1: {:#X}, imm: {:#X}",
        rd, rs1, imm
    );
    let extended_imm: u32 = sign_extend_12(imm);
    set_register(rd, registers, registers[rs1].wrapping_add(extended_imm));
}

fn execute_instruction(instruction: u32, registers: &mut [u32; 32], pc: &mut u32) {
    println!("instruction: {:#X}", instruction);
    let opcode = get_opcode(instruction);
    println!("opcode: {:#X}", opcode);
    let rd = get_rd(instruction);
    println!("rd: {:#X}", rd);
    let funct3 = get_funct3(instruction);
    println!("funct3: {:#X}", funct3);
    let rs1 = get_rs1(instruction);
    println!("rs1: {:#X}", rs1);
    let imm = get_imm(instruction);
    println!("imm: {:#X}", imm);

    match opcode {
        0b0010011 => match funct3 {
            0b000 => addi(rd, rs1, imm, registers),
            _ => panic!("unsupported funct3")
        },
        _ => panic!("unsupported opcode")
    }

    *pc += 4;
}

struct Emulator {
    registers: [u32; 32],
    pc: u32,
}

impl Emulator {
    fn new() -> Emulator {
        Emulator { registers: [0; 32], pc: 0 }
    }

    fn execute(&mut self, instruction: u32) {
        execute_instruction(instruction, &mut self.registers, &mut self.pc);
    }
}

fn main() {
    let mut emulator: Emulator = Emulator::new();

    emulator.registers[12] = 10;
    emulator.execute(0xfff60393);

    println!("register[7]: {}", emulator.registers[7]);
}

#[cfg(test)]
mod tests {
    use crate::Emulator;

    #[test]
    fn addi_adds_positive_immediate() {
        let mut emulator: Emulator = Emulator::new();
        emulator.registers[12] = 10;
        emulator.execute(0b0000_0000_0101_01100_000_00111_0010011);
        assert_eq!(emulator.registers[7], 15);
    }

    #[test]
    fn addi_adds_negative_immediate() {
        let mut emulator: Emulator = Emulator::new();
        emulator.registers[12] = 10;
        emulator.execute(0b1111_1111_1111_01100_000_00111_0010011);
        assert_eq!(emulator.registers[7], 9);
    }

    #[test]
    fn addi_handles_max_positive_immediate() {
        let mut emulator: Emulator = Emulator::new();
        emulator.execute(0b0111_1111_1111_00000_000_00111_0010011);
        assert_eq!(emulator.registers[7], 2047);
    }

    #[test]
    fn addi_handles_min_negative_immediate() {
        let mut emulator: Emulator = Emulator::new();
        emulator.execute(0b1000_0000_0000_00000_000_00111_0010011);
        assert_eq!(emulator.registers[7] as i32, -2048);
    }

    #[test]
    fn addi_does_not_modify_x0() {
        let mut emulator: Emulator = Emulator::new();
        emulator.registers[12] = 10;
        emulator.execute(0b0000_0000_0101_01100_000_00000_0010011);
        assert_eq!(emulator.registers[0], 0);
    }

    #[test]
    fn addi_wraps_on_overflow() {
        let mut emulator: Emulator = Emulator::new();
        emulator.registers[7] = 0x7fffffff;
        emulator.execute(0b0000_0000_0001_00111_000_00111_0010011);
        println!("{}", emulator.registers[7]);
        assert_eq!(emulator.registers[7], 0x80000000);
    }

    #[test]
    fn addi_wraps_on_underflow() {
        let mut emulator: Emulator = Emulator::new();
        emulator.registers[7] = 0;
        emulator.execute(0b1111_1111_1111_00111_000_00111_0010011);
        println!("{}", emulator.registers[7]);
        assert_eq!(emulator.registers[7], 0xFFFFFFFF);
    }
}
