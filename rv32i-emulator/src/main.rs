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
    return instruction >> 20;
}

fn get_rs2(instruction: u32) -> usize {
    return ((instruction >> 20) & 0b11111) as usize;
}

fn get_funct7(instruction: u32) -> u32 {
    return instruction >> 25;
}

fn set_register(index: usize, registers: &mut [u32; 32], value: u32) {
    match index {
        0 => {}
        1..32 => registers[index] = value,
        _ => panic!("not exists register"),
    }
}

fn sign_extend_12(unsigned_12: u32) -> u32 {
    return (((unsigned_12 << 20) as i32) >> 20) as u32;
}

fn addi(rd: usize, rs1: usize, imm: u32, registers: &mut [u32; 32]) {
    println!("[ADDI] rd: {:#X}, rs1: {:#X}, imm: {:#X}", rd, rs1, imm);
    let extended_imm: u32 = sign_extend_12(imm);
    set_register(rd, registers, registers[rs1].wrapping_add(extended_imm));
}

fn add(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[ADD] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1].wrapping_add(registers[rs2]));
}

fn sub(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[SUB] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1].wrapping_sub(registers[rs2]));
}

fn xor(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[XOR] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1] ^ registers[rs2]);
}

fn or(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[OR] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1] | registers[rs2]);
}

fn and(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[AND] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1] & registers[rs2]);
}

fn slt(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[SLT] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(
        rd,
        registers,
        ((registers[rs1] as i32) < (registers[rs2] as i32)) as u32,
    );
}

fn sltu(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[SLTU] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, (registers[rs1] < registers[rs2]) as u32);
}

fn sll(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[SLL] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1] << (registers[rs2] & 0b11111));
}

fn srl(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[SRL] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(rd, registers, registers[rs1] >> (registers[rs2] & 0b11111));
}

fn sra(rd: usize, rs1: usize, rs2: usize, registers: &mut [u32; 32]) {
    println!("[SRA] rd: {:#X}, rs1: {:#X}, rs2: {:#X}", rd, rs1, rs2);
    set_register(
        rd,
        registers,
        ((registers[rs1] as i32) >> (registers[rs2] & 0b11111)) as u32,
    );
}

fn execute_instruction(instruction: u32, registers: &mut [u32; 32], pc: &mut u32) {
    println!("instruction: {:#X}", instruction);
    let opcode = get_opcode(instruction);
    let rd = get_rd(instruction);
    let funct3 = get_funct3(instruction);
    let rs1 = get_rs1(instruction);
    let imm = get_imm(instruction);
    let rs2 = get_rs2(instruction);
    let funct7 = get_funct7(instruction);

    match opcode {
        0b0010011 => match funct3 {
            0b000 => addi(rd, rs1, imm, registers),
            _ => panic!("unsupported funct3"),
        },
        0b0110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => add(rd, rs1, rs2, registers),
                0b0100000 => sub(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b100 => match funct7 {
                0b0000000 => xor(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b110 => match funct7 {
                0b0000000 => or(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b111 => match funct7 {
                0b0000000 => and(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b010 => match funct7 {
                0b0000000 => slt(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b011 => match funct7 {
                0b0000000 => sltu(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b001 => match funct7 {
                0b0000000 => sll(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            0b101 => match funct7 {
                0b0000000 => srl(rd, rs1, rs2, registers),
                0b0100000 => sra(rd, rs1, rs2, registers),
                _ => panic!("unsupported funct7"),
            },
            _ => panic!("unsupported funct3"),
        },
        _ => panic!("unsupported opcode"),
    }
}

struct Emulator {
    registers: [u32; 32],
    pc: u32,
    memory: [u8; 2048],
}

impl Emulator {
    fn new() -> Emulator {
        Emulator {
            registers: [0; 32],
            pc: 0,
            memory: [0; 2048],
        }
    }

    fn fetch_instruction(&self) -> u32 {
        let pc = self.pc as usize;
        return self.memory[pc] as u32
            | ((self.memory[pc + 1] as u32) << 8)
            | ((self.memory[pc + 2] as u32) << 16)
            | ((self.memory[pc + 3] as u32) << 24);
    }

    fn next(&mut self) {
        let instruction = self.fetch_instruction();
        self.execute(instruction);
        self.pc += 4;
    }

    fn execute(&mut self, instruction: u32) {
        execute_instruction(instruction, &mut self.registers, &mut self.pc);
    }
}

fn main() {
    let mut emulator: Emulator = Emulator::new();
    // addi x1, x0, 5 = 00 50 00 93
    emulator.memory[0..4].copy_from_slice(&[0x93, 0x00, 0x50, 0x00]);
    // addi x2, x1, 10 = 00 a0 81 13
    emulator.memory[4..8].copy_from_slice(&[0x13, 0x81, 0xa0, 0x00]);
    // addi x3, x2, -3 = ff d1 01 93
    emulator.memory[8..12].copy_from_slice(&[0x93, 0x01, 0xd1, 0xff]);

    emulator.next();
    emulator.next();
    emulator.next();

    println!("{}", emulator.registers[3]);
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

    #[test]
    fn executes_multiple_addi_instructions() {
        let mut emulator = Emulator::new();

        // addi x1, x0, 5
        emulator.memory[0..4].copy_from_slice(&[0x93, 0x00, 0x50, 0x00]);

        // addi x2, x1, 10
        emulator.memory[4..8].copy_from_slice(&[0x13, 0x81, 0xa0, 0x00]);

        // addi x3, x2, -3
        emulator.memory[8..12].copy_from_slice(&[0x93, 0x01, 0xd1, 0xff]);

        emulator.next();
        emulator.next();
        emulator.next();

        assert_eq!(emulator.registers[1], 5);
        assert_eq!(emulator.registers[2], 15);
        assert_eq!(emulator.registers[3], 12);
        assert_eq!(emulator.pc, 12);
    }

    #[test]
    fn add_adds_two_registers() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 10;
        emulator.registers[2] = 20;

        // add x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0x81, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 30);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn add_wraps_on_overflow() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0xFFFF_FFFF;
        emulator.registers[2] = 1;

        // add x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0x81, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn add_does_not_modify_x0() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 10;
        emulator.registers[2] = 20;

        // add x0, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0x33, 0x80, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[0], 0);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn sub_subtracts_two_registers() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 30;
        emulator.registers[2] = 20;

        // sub x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0x81, 0x20, 0x40]);

        emulator.next();

        assert_eq!(emulator.registers[3], 10);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn sub_wraps_on_underflow() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0;
        emulator.registers[2] = 1;

        // sub x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0x81, 0x20, 0x40]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0xFFFF_FFFF);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn sub_does_not_modify_x0() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 30;
        emulator.registers[2] = 20;

        // sub x0, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0x33, 0x80, 0x20, 0x40]);

        emulator.next();

        assert_eq!(emulator.registers[0], 0);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn xor_applies_bitwise_xor() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0b1010;
        emulator.registers[2] = 0b1100;

        // xor x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xC1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0b0110);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn or_applies_bitwise_or() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0b1010;
        emulator.registers[2] = 0b1100;

        // or x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xE1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0b1110);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn and_applies_bitwise_and() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0b1010;
        emulator.registers[2] = 0b1100;

        // and x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xF1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0b1000);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn slt_compares_registers_as_signed() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0xFFFF_FFFF;
        emulator.registers[2] = 1;

        // slt x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xA1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 1);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn sltu_compares_registers_as_unsigned() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0xFFFF_FFFF;
        emulator.registers[2] = 1;

        // sltu x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xB1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn slt_writes_zero_when_condition_is_false() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 10;
        emulator.registers[2] = 5;

        // slt x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xA1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0);
    }

    #[test]
    fn sltu_writes_one_when_condition_is_true() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 5;
        emulator.registers[2] = 10;

        // sltu x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xB1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 1);
    }

    #[test]
    fn sll_shifts_left_logically() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0b1011;
        emulator.registers[2] = 2;

        // sll x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0x91, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0b101100);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn srl_shifts_right_logically() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0x8000_0000;
        emulator.registers[2] = 1;

        // srl x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xD1, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0x4000_0000);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn sra_shifts_right_arithmetically() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 0x8000_0000;
        emulator.registers[2] = 1;

        // sra x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0xD1, 0x20, 0x40]);

        emulator.next();

        assert_eq!(emulator.registers[3], 0xC000_0000);
        assert_eq!(emulator.pc, 4);
    }

    #[test]
    fn sll_uses_only_lower_five_bits_of_rs2() {
        let mut emulator = Emulator::new();

        emulator.registers[1] = 1;

        // 34 = 0b100010, lower 5 bits = 00010 = 2
        emulator.registers[2] = 34;

        // sll x3, x1, x2
        emulator.memory[0..4].copy_from_slice(&[0xB3, 0x91, 0x20, 0x00]);

        emulator.next();

        assert_eq!(emulator.registers[3], 4);
    }
}
