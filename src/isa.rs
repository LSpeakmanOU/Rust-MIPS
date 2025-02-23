use regex::Regex;
use std::fmt;
pub fn reg_as_str(reg_id: &u32) -> String{
    match reg_id{
        0 => "$0".to_string(),
        1 => "$at".to_string(),
        2 => "$v0".to_string(),
        3 => "$v1".to_string(),
        4 => "$a0".to_string(),
        5 => "$a1".to_string(),
        6 => "$a2".to_string(),
        7 => "$a3".to_string(),
        8 => "$t0".to_string(),
        9 => "$t1".to_string(),
        10 => "$t2".to_string(),
        11 => "$t3".to_string(),
        12 => "$t4".to_string(),
        13 => "$t5".to_string(),
        14 => "$t6".to_string(),
        15 => "$t7".to_string(),
        16 => "$s0".to_string(),
        17 => "$s1".to_string(),
        18 => "$s2".to_string(),
        19 => "$s3".to_string(),
        20 => "$s4".to_string(),
        21 => "$s5".to_string(),
        22 => "$s6".to_string(),
        23 => "$s7".to_string(),
        24 => "$t8".to_string(),
        25 => "$t9".to_string(),
        26 => "$k0".to_string(),
        27 => "$k1".to_string(),
        28 => "$gp".to_string(),
        29 => "$sp".to_string(),
        30 => "$ra".to_string(),
        31 => "$ra".to_string(),
        _ => "INVALID".to_string(),
    }
}
pub fn parse_reg(streg : &str) -> Result<u32, String>{
    let s = streg.replace(&['$',',',][..], "");
    match s.as_str(){
        "0" | "zero" => Ok(0),
        "1" | "at" => Ok(1),
        "2" | "v0" => Ok(2),
        "3" | "v1" => Ok(3),
        "4" | "a0" => Ok(4),
        "5" | "a1" => Ok(5),
        "6" | "a2" => Ok(6),
        "7" | "a3" => Ok(7),
        "8" | "t0" => Ok(8),
        "9" | "t1" => Ok(9),
        "10" | "t2" => Ok(10),
        "11" | "t3" => Ok(11),
        "12" | "t4" => Ok(12),
        "13" | "t5" => Ok(13),
        "14" | "t6" => Ok(14),
        "15" | "t7" => Ok(15),
        "16" | "s0" => Ok(16),
        "17" | "s1" => Ok(17),
        "18" | "s2" => Ok(18),
        "19" | "s3" => Ok(19),
        "20" | "s4" => Ok(20),
        "21" | "s5" => Ok(21),
        "22" | "s6" => Ok(22),
        "23" | "s7" => Ok(23),
        "24" | "t8" => Ok(24),
        "25" | "t9" => Ok(25),
        "26" | "k0" => Ok(26),
        "27" | "k1" => Ok(27),
        "28" | "gp" => Ok(28),
        "29" | "sp" => Ok(29),
        "30" | "31" | "ra" => Ok(30), // 30 and 31 point to the same place
        _ => Err(format!("Cannot parse register {}",s)),
    }
}
pub enum Instr{
    // Arithmetic instructions
    Add(u32, u32, u32),
    Sub(u32, u32, u32),
    Addi(u32, u32, u32),
    Addu(u32, u32, u32),
    Subu(u32, u32, u32),
    Addiu(u32, u32, u32),
    Mul(u32, u32, u32),
    Mult(u32, u32),
    Div(u32, u32),
    // Logical instructions
    And(u32, u32, u32),
    Or(u32, u32, u32),
    Andi(u32, u32, u32),
    Ori(u32, u32, u32),
    Sll(u32, u32, u32),
    Srl(u32, u32, u32),
    // Data Transfer
    Lw(u32, u32, u32),
    Sw(u32, u32, u32),
    Lui(u32, u32),
    La(u32, u32),
    Li(u32, u32),
    Mfhi(u32),
    Mflo(u32),
    Move(u32, u32),
    // Conditional Branch
    Beq(u32, u32, u32),
    Bne(u32, u32, u32),
    Bgt(u32, u32, u32),
    Bge(u32, u32, u32),
    Blt(u32, u32, u32),
    Ble(u32, u32, u32),
    // Comparison
    Slt(u32, u32, u32),
    Slti(u32, u32, u32),
    // Unconditional Jump
    Jump(u32),
    Jr(u32),
    Jal(u32),
}
impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Add(r1, r2, r3) => write!(f, "add {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Sub(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Addi(r1, r2, immd) => write!(f, "addi {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), immd),
            Instr::Addu(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Subu(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Addiu(r1, r2, immd) => write!(f, "addiu {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), immd),
            Instr::Mul(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Mult(rs, rt) => write!(f, "mult {}, {}",  reg_as_str(rs), reg_as_str(rt)),
            Instr::Div(rs, rt) => write!(f, "div {}, {}",  reg_as_str(rs), reg_as_str(rt)),
            Instr::And(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Or(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Andi(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Ori(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Sll(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Srl(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Lw(r1, r2, immd) => write!(f, "lw {}, {}({})",  reg_as_str(r1), immd, reg_as_str(r2)),
            Instr::Sw(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Lui(rt, immd) => write!(f, "lui {}, {}",  reg_as_str(rt), immd),
            Instr::La(rt, immd) => write!(f, "la {}, {}",  reg_as_str(rt), immd),
            Instr::Li(rt, immd) => write!(f, "li {}, {}",  reg_as_str(rt), immd),
            Instr::Mfhi(r1) => write!(f, "mfhi {}",  reg_as_str(r1)),
            Instr::Mflo(r1) => write!(f, "mflo {}",  reg_as_str(r1)),
            Instr::Move(rt, rs) => write!(f, "move {}, {}",  reg_as_str(rt), reg_as_str(rs)),
            Instr::Beq(r1, r2, addr) => write!(f, "beq {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), addr),
            Instr::Bne(r1, r2, addr) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), addr),
            Instr::Bgt(r1, r2, addr) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), addr),
            Instr::Bge(r1, r2, addr) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), addr),
            Instr::Blt(r1, r2, addr) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), addr),
            Instr::Ble(r1, r2, addr) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), addr),
            Instr::Slt(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Slti(r1, r2, r3) => write!(f, "sub {}, {}, {}",  reg_as_str(r1), reg_as_str(r2), reg_as_str(r3)),
            Instr::Jump(addr) => write!(f, "j {}",  addr),
            Instr::Jr(rt) => write!(f, "jr {}",  reg_as_str(rt)),
            Instr::Jal(addr) => write!(f, "jal {}",  addr),
        }
    }
}
impl Instr {
    
    pub fn from_str(line: &String) -> Result<Instr, String> {
        let tokens: Vec<&str> = regex::Regex::new(r"([\s()$,]+|#.*)").unwrap()
        .split(line).filter(|s| !s.is_empty()).collect();
        match tokens[0]{
            "add" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                let r3: u32 = parse_reg(tokens[3])?;
                return Ok(Instr::Add(r1, r2, r3));
            }
            "sub" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                let r3: u32 = parse_reg(tokens[3])?;
                return Ok(Instr::Sub(r1, r2, r3));
            }
            "addu" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                let r3: u32 = parse_reg(tokens[3])?;
                return Ok(Instr::Addu(r1, r2, r3));
            }
            "subu" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                let r3: u32 = parse_reg(tokens[3])?;
                return Ok(Instr::Subu(r1, r2, r3));
            }
            "addi" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                let immd: u32 = tokens[3].parse::<u32>().unwrap();
                return Ok(Instr::Addi(r1, r2, immd));
            }
            "addiu" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                let immd: u32 = tokens[3].parse::<u32>().unwrap();
                return Ok(Instr::Addiu(r1, r2, immd));
            }
            "lw" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let immd: u32 = tokens[2].parse::<u32>().unwrap();
                let r2: u32 = parse_reg(tokens[3])?;
                return Ok(Instr::Lw(r1, r2, immd));
            }
            "div" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                return Ok(Instr::Div(r1, r2));
            }
            "mult" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                return Ok(Instr::Mult(r1, r2));
            }
            
            _ => {return Err(format!("Could Not parse {}",line))}
        }
    }
}
#[derive(Debug)]
pub enum ParsedInstr {
    Label(String),
    I(Instr),
    NOP,
    // Conditional Branch
    Beq(u32, u32, String),
    Bne(u32, u32, String),
    Bgt(u32, u32, String),
    Bge(u32, u32, String),
    Blt(u32, u32, String),
    Ble(u32, u32, String),
    // Unconditional Jump
    Jump(String),
    Jr(String),
    Jal(String),
}

impl ParsedInstr {
    pub fn from_str(line: &String) -> Result<ParsedInstr, String> {
        let tokens: Vec<&str> = regex::Regex::new(r"([\s()$,]+|#.*)").unwrap()
        .split(line).filter(|s| !s.is_empty()).collect();
        // Skip blank lines
        if tokens.len() == 0{
            return Ok(ParsedInstr::NOP);
        }
        match tokens[0]{
            "beq" => {
                let r1: u32 = parse_reg(tokens[1])?;
                let r2: u32 = parse_reg(tokens[2])?;
                return Ok(ParsedInstr::Beq(r1,r2,tokens[3].to_string()));
            }
            "jal" => {
                return Ok(ParsedInstr::Jal(tokens[1].to_string()));
            }
            // If not one of the clearly understood label-based instructions its a label or another command
            _ => {
                let label_regex = regex::Regex::new(r"^[A-Za-z0-9_]+:$").unwrap();
                if label_regex.is_match(tokens[0]){
                    let label = tokens[0].replace(":", "");
                    return Ok(ParsedInstr::Label(label));
                }
                let instr = Instr::from_str(line)?;
                return Ok(ParsedInstr::I(instr));
            }
        }
    }
}
