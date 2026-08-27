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

fn get_signed_imm(instruction: u32) -> i32 {
    let casted_imm = (instruction >> 20) as i32; // = (instruction >> 20) & 0b111111111111
    if casted_imm < 2048 {
        return casted_imm;
    } else {
        return casted_imm - 4096; // 4096 = 2^12
    }
}

fn set_register(index: usize, registers: &mut [i32; 32], value: i32) {
    match index {
        0 => {}
        1..32 => registers[index] = value,
        _ => {}
    }
}

fn addi(rd: usize, rs1: usize, signed_imm: i32, registers: &mut [i32; 32]) {
    println!(
        "[ADDI] rd: {}, rs1: {}, signed_imm: {}",
        rd, rs1, signed_imm
    );
    set_register(rd, registers, registers[rs1] + signed_imm);
}

fn execute(instruction: u32, registers: &mut [i32; 32], pc: &mut u32) {
    println!("instruction: {:#X}", instruction);
    let opcode = get_opcode(instruction);
    println!("opcode: {}", opcode);
    let rd = get_rd(instruction);
    println!("rd: {}", rd);
    let funct3 = get_funct3(instruction);
    println!("funct3: {}", funct3);
    let rs1 = get_rs1(instruction);
    println!("rs1: {}", rs1);
    let signed_imm = get_signed_imm(instruction);
    println!("signed_imm: {}", signed_imm);

    match opcode {
        0b0010011 => match funct3 {
            0b000 => addi(rd, rs1, signed_imm, registers),
            _ => {} // no-op
        },
        _ => {} // no-op
    }

    *pc += 4;
}

fn main() {
    let mut registers: [i32; 32] = [0; 32];
    let mut pc: u32 = 0;

    let instruction: u32 = 0xfff60393;

    registers[12] = 10;
    execute(instruction, &mut registers, &mut pc);

    println!("register[7]: {}", registers[7])
}
