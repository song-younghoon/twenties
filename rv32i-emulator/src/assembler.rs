fn encode_i_type_instruction(opcode: u32, funct3: u32, rd: u32, rs1: u32, imm: i32) -> u32 {
    if rd >= 32 {
        panic!("rd is out of range")
    }
    if rs1 >= 32 {
        panic!("rs1 is out of range")
    }
    if imm < -2048 || imm >= 2048 {
        panic!("imm is out of range")
    }

    return opcode | (funct3 << 12) | (rd << 7) | (rs1 << 15) | ((imm as u32) << 20);
}

fn encode_addi(rd: u32, rs1: u32, imm: i32) -> u32 {
    return encode_i_type_instruction(0b0010011, 0b000, rd, rs1, imm);
}

fn encode_r_type_instruction(
    opcode: u32,
    funct3: u32,
    funct7: u32,
    rd: u32,
    rs1: u32,
    rs2: u32,
) -> u32 {
    if rd >= 32 {
        panic!("rd is out of range")
    }
    if rs1 >= 32 {
        panic!("rs1 is out of range")
    }
    if rs2 >= 32 {
        panic!("rs2 is out of range")
    }

    return opcode | (funct3 << 12) | (rd << 7) | (rs1 << 15) | (rs2 << 20) | (funct7 << 25);
}

fn encode_add(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b000, 0b0000000, rd, rs1, rs2);
}

fn encode_sub(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b000, 0b0100000, rd, rs1, rs2);
}

fn encode_xor(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b100, 0b0000000, rd, rs1, rs2);
}

fn encode_or(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b110, 0b0000000, rd, rs1, rs2);
}

fn encode_and(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b111, 0b0000000, rd, rs1, rs2);
}

fn encode_slt(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b010, 0b0000000, rd, rs1, rs2);
}

fn encode_sltu(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b011, 0b0000000, rd, rs1, rs2);
}

fn encode_sll(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b001, 0b0000000, rd, rs1, rs2);
}

fn encode_srl(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b101, 0b0000000, rd, rs1, rs2);
}

fn encode_sra(rd: u32, rs1: u32, rs2: u32) -> u32 {
    return encode_r_type_instruction(0b0110011, 0b101, 0b0100000, rd, rs1, rs2);
}

fn encode_b_type_instruction(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
    if rs1 >= 32 {
        panic!("rs1 is out of range")
    }
    if rs2 >= 32 {
        panic!("rs2 is out of range")
    }
    if imm < -4096 || imm >= 4096 {
        panic!("imm is out of range")
    }
    if imm % 2 != 0 {
        panic!("imm must be even number")
    }
    let casted_imm = imm as u32;
    let imm_segment1 = (casted_imm >> 11) & 0b1;
    let imm_segment2 = (casted_imm >> 1) & 0b1111;
    let imm_segment3 = (casted_imm >> 5) & 0b111111;
    let imm_segment4 = (casted_imm >> 12) & 0b1;

    return opcode
        | (funct3 << 12)
        | (rs1 << 15)
        | (rs2 << 20)
        | (imm_segment1 << 7)
        | (imm_segment2 << 8)
        | (imm_segment3 << 25)
        | (imm_segment4 << 31);
}

fn encode_beq(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return encode_b_type_instruction(0b1100011, 0b000, rs1, rs2, imm);
}

fn encode_bne(rs1: u32, rs2: u32, imm: i32) -> u32 {
    return encode_b_type_instruction(0b1100011, 0b001, rs1, rs2, imm);
}

#[cfg(test)]
mod tests {
    use crate::{assembler::*, *};

    #[test]
    fn encode_addi_positive_immediate() {
        assert_eq!(encode_addi(1, 0, 5), 0x0050_0093);
    }

    #[test]
    fn encode_addi_negative_immediate() {
        assert_eq!(encode_addi(7, 12, -1), 0xFFF6_0393);
    }

    #[test]
    fn encode_addi_max_positive_immediate() {
        assert_eq!(encode_addi(7, 0, 2047), 0x7FF0_0393);
    }

    #[test]
    fn encode_addi_min_negative_immediate() {
        assert_eq!(encode_addi(7, 0, -2048), 0x8000_0393);
    }

    #[test]
    fn encode_add_instruction() {
        assert_eq!(encode_add(3, 1, 2), 0x0020_81B3);
    }

    #[test]
    fn encode_sub_instruction() {
        assert_eq!(encode_sub(3, 1, 2), 0x4020_81B3);
    }

    #[test]
    fn encode_xor_instruction() {
        assert_eq!(encode_xor(3, 1, 2), 0x0020_C1B3);
    }

    #[test]
    fn encode_or_instruction() {
        assert_eq!(encode_or(3, 1, 2), 0x0020_E1B3);
    }

    #[test]
    fn encode_and_instruction() {
        assert_eq!(encode_and(3, 1, 2), 0x0020_F1B3);
    }

    #[test]
    fn encode_slt_instruction() {
        assert_eq!(encode_slt(3, 1, 2), 0x0020_A1B3);
    }

    #[test]
    fn encode_sltu_instruction() {
        assert_eq!(encode_sltu(3, 1, 2), 0x0020_B1B3);
    }

    #[test]
    fn encode_sll_instruction() {
        assert_eq!(encode_sll(3, 1, 2), 0x0020_91B3);
    }

    #[test]
    fn encode_srl_instruction() {
        assert_eq!(encode_srl(3, 1, 2), 0x0020_D1B3);
    }

    #[test]
    fn encode_sra_instruction() {
        assert_eq!(encode_sra(3, 1, 2), 0x4020_D1B3);
    }

    #[test]
    #[should_panic(expected = "rd is out of range")]
    fn encode_r_type_rejects_invalid_rd() {
        encode_add(32, 1, 2);
    }

    #[test]
    #[should_panic(expected = "rs1 is out of range")]
    fn encode_r_type_rejects_invalid_rs1() {
        encode_add(3, 32, 2);
    }

    #[test]
    #[should_panic(expected = "rs2 is out of range")]
    fn encode_r_type_rejects_invalid_rs2() {
        encode_add(3, 1, 32);
    }

    #[test]
    fn encode_beq_forward_branch() {
        assert_eq!(encode_beq(1, 2, 8), 0x0020_8463);
    }

    #[test]
    fn encode_bne_forward_branch() {
        assert_eq!(encode_bne(1, 2, 8), 0x0020_9463);
    }

    #[test]
    fn encode_beq_backward_branch() {
        assert_eq!(encode_beq(1, 2, -8), 0xFE20_8CE3);
    }

    #[test]
    fn encode_bne_backward_branch() {
        assert_eq!(encode_bne(1, 2, -8), 0xFE20_9CE3);
    }

    #[test]
    fn encode_beq_minimum_offset() {
        let instruction = encode_beq(1, 2, -4096);
        let raw_imm = get_b_type_imm(instruction);
        assert_eq!(sign_extend_13(raw_imm), (-4096i32) as u32);
    }

    #[test]
    fn encode_beq_maximum_offset() {
        let instruction = encode_beq(1, 2, 4094);
        let raw_imm = get_b_type_imm(instruction);
        assert_eq!(sign_extend_13(raw_imm), 4094);
    }

    #[test]
    #[should_panic(expected = "imm is out of range")]
    fn encode_beq_rejects_too_large_positive_offset() {
        encode_beq(1, 2, 4096);
    }

    #[test]
    #[should_panic(expected = "imm is out of range")]
    fn encode_beq_rejects_too_small_negative_offset() {
        encode_beq(1, 2, -4098);
    }

    #[test]
    #[should_panic(expected = "imm must be even number")]
    fn encode_beq_rejects_odd_offset() {
        encode_beq(1, 2, 7);
    }

    #[test]
    #[should_panic(expected = "rs1 is out of range")]
    fn encode_beq_rejects_invalid_rs1() {
        encode_beq(32, 2, 8);
    }

    #[test]
    #[should_panic(expected = "rs2 is out of range")]
    fn encode_beq_rejects_invalid_rs2() {
        encode_beq(1, 32, 8);
    }

    #[test]
    fn beq_encoding_round_trip_preserves_operands_and_offset() {
        let encoded = encode_beq(7, 12, -128);

        assert_eq!(get_rs1(encoded), 7);
        assert_eq!(get_rs2(encoded), 12);

        let imm = get_b_type_imm(encoded);
        assert_eq!(sign_extend_13(imm), (-128i32) as u32);
    }

    #[test]
    #[should_panic(expected = "imm must be even number")]
    fn encode_beq_rejects_negative_odd_offset() {
        encode_beq(1, 2, -3);
    }
}
