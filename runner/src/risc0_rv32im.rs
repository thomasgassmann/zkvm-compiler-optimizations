// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::panic;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DecodedInstruction {
    pub insn: u32,
    top_bit: u32,
    func7: u32,
    rs2: u32,
    rs1: u32,
    func3: u32,
    rd: u32,
    opcode: u32,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum InsnKind {
    Add = 0,  // major: 0, minor: 0
    Sub = 1,  // major: 0, minor: 1
    Xor = 2,  // major: 0, minor: 2
    Or = 3,   // major: 0, minor: 3
    And = 4,  // major: 0, minor: 4
    Slt = 5,  // major: 0, minor: 5
    SltU = 6, // major: 0, minor: 6
    AddI = 7, // major: 0, minor: 7

    XorI = 8,   // major: 1, minor: 0
    OrI = 9,    // major: 1, minor: 1
    AndI = 10,  // major: 1, minor: 2
    SltI = 11,  // major: 1, minor: 3
    SltIU = 12, // major: 1, minor: 4
    Beq = 13,   // major: 1, minor: 5
    Bne = 14,   // major: 1, minor: 6
    Blt = 15,   // major: 1, minor: 7

    Bge = 16,   // major: 2, minor: 0
    BltU = 17,  // major: 2, minor: 1
    BgeU = 18,  // major: 2, minor: 2
    Jal = 19,   // major: 2, minor: 3
    JalR = 20,  // major: 2, minor: 4
    Lui = 21,   // major: 2, minor: 5
    Auipc = 22, // major: 2, minor: 6

    Sll = 24,    // major: 3, minor: 0
    SllI = 25,   // major: 3, minor: 1
    Mul = 26,    // major: 3, minor: 2
    MulH = 27,   // major: 3, minor: 3
    MulHSU = 28, // major: 3, minor: 4
    MulHU = 29,  // major: 3, minor: 5

    Srl = 32,  // major: 4, minor: 0
    Sra = 33,  // major: 4, minor: 1
    SrlI = 34, // major: 4, minor: 2
    SraI = 35, // major: 4, minor: 3
    Div = 36,  // major: 4, minor: 4
    DivU = 37, // major: 4, minor: 5
    Rem = 38,  // major: 4, minor: 6
    RemU = 39, // major: 4, minor: 7

    Lb = 40,  // major: 5, minor: 0
    Lh = 41,  // major: 5, minor: 1
    Lw = 42,  // major: 5, minor: 2
    LbU = 43, // major: 5, minor: 3
    LhU = 44, // major: 5, minor: 4

    Sb = 48, // major: 6, minor: 0
    Sh = 49, // major: 6, minor: 1
    Sw = 50, // major: 6, minor: 2

    Eany = 56, // major: 7, minor: 0
    Mret = 57, // major: 7, minor: 1

    Invalid = 255,
}

#[allow(dead_code)]
impl DecodedInstruction {
    pub fn new(insn: u32) -> Self {
        Self {
            insn,
            top_bit: (insn & 0x80000000) >> 31,
            func7: (insn & 0xfe000000) >> 25,
            rs2: (insn & 0x01f00000) >> 20,
            rs1: (insn & 0x000f8000) >> 15,
            func3: (insn & 0x00007000) >> 12,
            rd: (insn & 0x00000f80) >> 7,
            opcode: insn & 0x0000007f,
        }
    }

    fn imm_b(&self) -> u32 {
        (self.top_bit * 0xfffff000)
            | ((self.rd & 1) << 11)
            | ((self.func7 & 0x3f) << 5)
            | (self.rd & 0x1e)
    }

    fn imm_i(&self) -> u32 {
        (self.top_bit * 0xfffff000) | (self.func7 << 5) | self.rs2
    }

    fn imm_s(&self) -> u32 {
        (self.top_bit * 0xfffff000) | (self.func7 << 5) | self.rd
    }

    fn imm_j(&self) -> u32 {
        (self.top_bit * 0xfff00000)
            | (self.rs1 << 15)
            | (self.func3 << 12)
            | ((self.rs2 & 1) << 11)
            | ((self.func7 & 0x3f) << 5)
            | (self.rs2 & 0x1e)
    }

    fn imm_u(&self) -> u32 {
        self.insn & 0xfffff000
    }
}

pub fn get_insn_kind(word: u32) -> InsnKind {
    let decoded = DecodedInstruction::new(word);

    match (decoded.opcode, decoded.func3, decoded.func7) {
        // R-format arithmetic ops
        (0b0110011, 0b000, 0b0000000) => InsnKind::Add,
        (0b0110011, 0b000, 0b0100000) => InsnKind::Sub,
        (0b0110011, 0b001, 0b0000000) => InsnKind::Sll,
        (0b0110011, 0b010, 0b0000000) => InsnKind::Slt,
        (0b0110011, 0b011, 0b0000000) => InsnKind::SltU,
        (0b0110011, 0b101, 0b0000000) => InsnKind::Srl,
        (0b0110011, 0b100, 0b0000000) => InsnKind::Xor,
        (0b0110011, 0b101, 0b0100000) => InsnKind::Sra,
        (0b0110011, 0b110, 0b0000000) => InsnKind::Or,
        (0b0110011, 0b111, 0b0000000) => InsnKind::And,
        (0b0110011, 0b000, 0b0000001) => InsnKind::Mul,
        (0b0110011, 0b001, 0b0000001) => InsnKind::MulH,
        (0b0110011, 0b010, 0b0000001) => InsnKind::MulHSU,
        (0b0110011, 0b011, 0b0000001) => InsnKind::MulHU,
        (0b0110011, 0b100, 0b0000001) => InsnKind::Div,
        (0b0110011, 0b101, 0b0000001) => InsnKind::DivU,
        (0b0110011, 0b110, 0b0000001) => InsnKind::Rem,
        (0b0110011, 0b111, 0b0000001) => InsnKind::RemU,
        // I-format arithmetic ops
        (0b0010011, 0b000, _) => InsnKind::AddI,
        (0b0010011, 0b001, 0b0000000) => InsnKind::SllI,
        (0b0010011, 0b010, _) => InsnKind::SltI,
        (0b0010011, 0b011, _) => InsnKind::SltIU,
        (0b0010011, 0b100, _) => InsnKind::XorI,
        (0b0010011, 0b101, 0b0000000) => InsnKind::SrlI,
        (0b0010011, 0b101, 0b0100000) => InsnKind::SraI,
        (0b0010011, 0b110, _) => InsnKind::OrI,
        (0b0010011, 0b111, _) => InsnKind::AndI,
        // I-format memory loads
        (0b0000011, 0b000, _) => InsnKind::Lb,
        (0b0000011, 0b001, _) => InsnKind::Lh,
        (0b0000011, 0b010, _) => InsnKind::Lw,
        (0b0000011, 0b100, _) => InsnKind::LbU,
        (0b0000011, 0b101, _) => InsnKind::LhU,
        // S-format memory stores
        (0b0100011, 0b000, _) => InsnKind::Sb,
        (0b0100011, 0b001, _) => InsnKind::Sh,
        (0b0100011, 0b010, _) => InsnKind::Sw,
        // U-format lui
        (0b0110111, _, _) => InsnKind::Lui,
        // U-format auipc
        (0b0010111, _, _) => InsnKind::Auipc,
        // B-format branch
        (0b1100011, 0b000, _) => InsnKind::Beq,
        (0b1100011, 0b001, _) => InsnKind::Bne,
        (0b1100011, 0b100, _) => InsnKind::Blt,
        (0b1100011, 0b101, _) => InsnKind::Bge,
        (0b1100011, 0b110, _) => InsnKind::BltU,
        (0b1100011, 0b111, _) => InsnKind::BgeU,
        // J-format jal
        (0b1101111, _, _) => InsnKind::Jal,
        // I-format jalr
        (0b1100111, _, _) => InsnKind::JalR,
        // System instruction
        (0b1110011, 0b000, 0b0011000) => InsnKind::Mret,
        (0b1110011, 0b000, 0b0000000) => InsnKind::Eany,
        _ => panic!("Unknown instruction: {:#010x}", word),
    }
}

pub const REG_ZERO: usize = 0; // zero constant
pub const REG_MAX: usize = 32; // maximum number of registers
struct Register(u32);

const REG_ALIASES: [&str; REG_MAX] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", REG_ALIASES[self.0 as usize])
    }
}

pub fn disasm(kind: InsnKind, decoded: &DecodedInstruction) -> String {
    let (rd, rs1, rs2) = (
        Register(decoded.rd),
        Register(decoded.rs1),
        Register(decoded.rs2),
    );
    match kind {
        InsnKind::Invalid => "illegal".to_string(),
        InsnKind::Add => format!("add {rd}, {rs1}, {rs2}"),
        InsnKind::Sub => format!("sub {rd}, {rs1}, {rs2}"),
        InsnKind::Xor => format!("xor {rd}, {rs1}, {rs2}"),
        InsnKind::Or => format!("or {rd}, {rs1}, {rs2}"),
        InsnKind::And => format!("and {rd}, {rs1}, {rs2}"),
        InsnKind::Sll => format!("sll {rd}, {rs1}, {rs2}"),
        InsnKind::Srl => format!("srl {rd}, {rs1}, {rs2}"),
        InsnKind::Sra => format!("sra {rd}, {rs1}, {rs2}"),
        InsnKind::Slt => format!("slt {rd}, {rs1}, {rs2}"),
        InsnKind::SltU => format!("sltu {rd}, {rs1}, {rs2}"),
        InsnKind::AddI => {
            if rs1.0 == REG_ZERO as u32 {
                format!("li {rd}, {}", decoded.imm_i() as i32)
            } else {
                format!("addi {rd}, {rs1}, {}", decoded.imm_i() as i32)
            }
        }
        InsnKind::XorI => format!("xori {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::OrI => format!("ori {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::AndI => format!("andi {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::SllI => format!("slli {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::SrlI => format!("srli {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::SraI => format!("srai {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::SltI => format!("slti {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::SltIU => format!("sltiu {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::Beq => format!("beq {rs1}, {rs2}, {}", decoded.imm_b() as i32),
        InsnKind::Bne => format!("bne {rs1}, {rs2}, {}", decoded.imm_b() as i32),
        InsnKind::Blt => format!("blt {rs1}, {rs2}, {}", decoded.imm_b() as i32),
        InsnKind::Bge => format!("bge {rs1}, {rs2}, {}", decoded.imm_b() as i32),
        InsnKind::BltU => format!("bltu {rs1}, {rs2}, {}", decoded.imm_b() as i32),
        InsnKind::BgeU => format!("bgeu {rs1}, {rs2}, {}", decoded.imm_b() as i32),
        InsnKind::Jal => format!("jal {rd}, {}", decoded.imm_j() as i32),
        InsnKind::JalR => format!("jalr {rd}, {rs1}, {}", decoded.imm_i() as i32),
        InsnKind::Lui => format!("lui {rd}, {:#010x}", decoded.imm_u() >> 12),
        InsnKind::Auipc => format!("auipc {rd}, {:#010x}", decoded.imm_u() >> 12),
        InsnKind::Mul => format!("mul {rd}, {rs1}, {rs2}"),
        InsnKind::MulH => format!("mulh {rd}, {rs1}, {rs2}"),
        InsnKind::MulHSU => format!("mulhsu {rd}, {rs1}, {rs2}"),
        InsnKind::MulHU => format!("mulhu {rd}, {rs1}, {rs2}"),
        InsnKind::Div => format!("div {rd}, {rs1}, {rs2}"),
        InsnKind::DivU => format!("divu {rd}, {rs1}, {rs2}"),
        InsnKind::Rem => format!("rem {rd}, {rs1}, {rs2}"),
        InsnKind::RemU => format!("remu {rd}, {rs1}, {rs2}"),
        InsnKind::Lb => format!("lb {rd}, {}({rs1})", decoded.imm_i() as i32),
        InsnKind::Lh => format!("lh {rd}, {}({rs1})", decoded.imm_i() as i32),
        InsnKind::Lw => format!("lw {rd}, {}({rs1})", decoded.imm_i() as i32),
        InsnKind::LbU => format!("lbu {rd}, {}({rs1})", decoded.imm_i() as i32),
        InsnKind::LhU => format!("lhu {rd}, {}({rs1})", decoded.imm_i() as i32),
        InsnKind::Sb => format!("sb {rs2}, {}({rs1})", decoded.imm_s() as i32),
        InsnKind::Sh => format!("sh {rs2}, {}({rs1})", decoded.imm_s() as i32),
        InsnKind::Sw => format!("sw {rs2}, {}({rs1})", decoded.imm_s() as i32),
        InsnKind::Eany => match decoded.rs2 {
            0 => "ecall".to_string(),
            1 => "ebreak".to_string(),
            _ => "illegal eany".to_string(),
        },
        InsnKind::Mret => "mret".to_string(),
    }
}
