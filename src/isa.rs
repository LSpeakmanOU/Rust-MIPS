use regex::Regex;
use std::fmt;
pub fn reg_as_str(reg_id: &u32) -> String{
    // Register names to map
    let reg_names = vec![
        "$0", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", 
        "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6", "$t7",
        "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7",
        "$t8", "$t9", "$k0", "$k1", "$gp", "$sp", "$fp", "$ra"
    ];
    // Attempt to map id to register name, if possible then map otherwise, return INVALID
    if let Some(&reg_str) = reg_names.get(*reg_id as usize) {
        reg_str.to_string()
    } else {
        "INVALID".to_string()
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
        "30" | "fp" => Ok(30),
        "31" | "ra" => Ok(31),
        _ => Err(format!("Cannot parse register {}",s)),
    }
}
#[derive(Clone, Copy)]
pub enum Instr{
    // Arithmetic instructions
    Add{rd: u32, rs: u32, rt: u32},
    Sub{rd: u32, rs: u32, rt: u32},
    Addu{rd: u32, rs: u32, rt: u32},
    Subu{rd: u32, rs: u32, rt: u32},
    Addi{rt: u32, rs: u32, immd: u32},
    Addiu{rt: u32, rs: u32, immd: u32},
    // Multiplication/Division instructions
    Mul{rd: u32, rs: u32, rt: u32},
    Mult{rs: u32, rt: u32},
    Div{rs: u32, rt: u32},
    // Logical instructions
    And{rd: u32, rs: u32, rt: u32},
    Or{rd: u32, rs: u32, rt: u32},
    Andi{rt: u32, rs: u32, immd: u32},
    Ori{rt: u32, rs: u32, immd: u32},
    // Shift instructions
    Sll{rd: u32, rs: u32, shamt: u32},
    Srl{rd: u32, rs: u32, shamt: u32},
    // Data Transfer
    Lw{rt: u32, rs: u32, immd: u32},
    Sw{rt: u32, rs: u32, immd: u32},
    Lui{rt: u32, immd: u32},
    La{rt: u32, addr: u32}, // Pseudo-instruction
    Li{rt: u32, immd: u32}, // Pseudo-instruction
    Mfhi{rd: u32},
    Mflo{rd: u32},
    Move{rs: u32, rt: u32}, // Pseudo-instruction
    // Conditional Branch
    Beq{rt: u32, rs: u32, rel_addr: i32},
    Bne{rt: u32, rs: u32, rel_addr: i32},
    Bgt{rt: u32, rs: u32, rel_addr: i32}, // Pseudo-instruction
    Bge{rt: u32, rs: u32, rel_addr: i32}, // Pseudo-instruction
    Blt{rt: u32, rs: u32, rel_addr: i32}, // Pseudo-instruction
    Ble{rt: u32, rs: u32, rel_addr: i32}, // Pseudo-instruction
    // Comparison
    Slt{rd: u32, rs: u32, rt: u32},
    Slti{rt: u32, rs: u32, immd: u32},
    Sltiu{rt: u32, rs: u32, immd: u32},
    // Unconditional Jump
    Jump{addr: u32},
    Jr{rd: u32},
    Jal{addr: u32},
}
impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instr::Add{rd, rs, rt} => write!(f, "add {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Sub{rd, rs, rt} => write!(f, "sub {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Addi{rt, rs, immd} => write!(f, "addi {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), immd),
            Instr::Addu{rd, rs, rt} => write!(f, "addu {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Subu{rd, rs, rt} => write!(f, "subu {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Addiu{rt, rs, immd} => write!(f, "addiu {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), immd),
            Instr::Mul{rd, rs, rt} => write!(f, "mul {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Mult{rs, rt} => write!(f, "mult {}, {}",  reg_as_str(rs), reg_as_str(rt)),
            Instr::Div{rs, rt} => write!(f, "div {}, {}",  reg_as_str(rs), reg_as_str(rt)),
            Instr::And{rd, rs, rt} => write!(f, "and {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Or{rd, rs, rt} => write!(f, "or {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Andi{rt, rs, immd} => write!(f, "andi {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), immd),
            Instr::Ori{rt, rs, immd} => write!(f, "ori {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), immd),
            Instr::Sll{rd, rs, shamt} => write!(f, "sll {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), shamt),
            Instr::Srl{rd, rs, shamt} => write!(f, "srl {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), shamt),
            Instr::Lw{rt, rs, immd} => write!(f, "lw {}, {}({})",  reg_as_str(rt), immd, reg_as_str(rs)),
            Instr::Sw{rt, rs, immd} => write!(f, "sw {}, {}({})",  reg_as_str(rt), immd, reg_as_str(rs)),
            Instr::Lui{rt, immd} => write!(f, "lui {}, {}",  reg_as_str(rt), immd),
            Instr::La{rt, addr} => write!(f, "la {}, {}",  reg_as_str(rt), addr),
            Instr::Li{rt, immd} => write!(f, "li {}, {}",  reg_as_str(rt), immd),
            Instr::Mfhi{rd} => write!(f, "mfhi {}",  reg_as_str(rd)),
            Instr::Mflo{rd} => write!(f, "mflo {}",  reg_as_str(rd)),
            Instr::Move{rs, rt} => write!(f, "move {}, {}",  reg_as_str(rs), reg_as_str(rt)),
            Instr::Beq{rt, rs, rel_addr} => write!(f, "beq {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), rel_addr),
            Instr::Bne{rt, rs, rel_addr} => write!(f, "bne {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), rel_addr),
            Instr::Bgt{rt, rs, rel_addr} => write!(f, "bgt {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), rel_addr),
            Instr::Bge{rt, rs, rel_addr} => write!(f, "bge {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), rel_addr),
            Instr::Blt{rt, rs, rel_addr} => write!(f, "blt {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), rel_addr),
            Instr::Ble{rt, rs, rel_addr} => write!(f, "ble {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), rel_addr),
            Instr::Slt{rd, rs, rt} => write!(f, "slt {}, {}, {}",  reg_as_str(rd), reg_as_str(rs), reg_as_str(rt)),
            Instr::Slti{rt, rs, immd} => write!(f, "slti {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), immd),
            Instr::Sltiu{rt, rs, immd} => write!(f, "slti {}, {}, {}",  reg_as_str(rt), reg_as_str(rs), immd),
            Instr::Jump{addr} => write!(f, "j {}",  addr),
            Instr::Jr{rd} => write!(f, "jr {}",  reg_as_str(rd)),
            Instr::Jal{addr} => write!(f, "jal {}",  addr),
        }
    }
}
impl Instr {
    
    pub fn from_str(line: &String) -> Result<Instr, String> {
        let tokens: Vec<&str> = regex::Regex::new(r"([\s()$,]+|#.*)").unwrap()
        .split(line).filter(|s| !s.is_empty()).collect();
        let parse_immediate = |s: &str| s.parse::<u32>().unwrap();
        
        let parse_two_regs = |tokens: &[&str]| -> Result<(u32, u32), String> {
            Ok((parse_reg(tokens[1])?, parse_reg(tokens[2])?))
        };

        let parse_three_regs = |tokens: &[&str]| -> Result<(u32, u32, u32), String> {
            Ok((parse_reg(tokens[1])?, parse_reg(tokens[2])?, parse_reg(tokens[3])?))
        };
        if tokens.len() == 0{
            return Err("No Tokens to parse!".to_string());
        }
        match tokens[0]{
            "add" | "addu" | "sub" | "subu" | "mul" | "and" | "or" | "slt" =>{
                if tokens.len() < 4{
                    return Err("Cannot parse 3 register R instruction!".to_string());
                }
                let (rd, rs, rt) = parse_three_regs(&tokens)?;
                Ok(match tokens[0] {
                    "add" => Instr::Add{rd, rs, rt},
                    "addu" => Instr::Addu{rd, rs, rt},
                    "sub" => Instr::Sub{rd, rs, rt},
                    "subu" => Instr::Subu{rd, rs, rt},
                    "mul" => Instr::Mul{rd, rs, rt},
                    "and" => Instr::And{rd, rs, rt},
                    "or" => Instr::Or{rd, rs, rt},
                    "slt" => Instr::Slt{rd, rs, rt},
                    _=> unreachable!()
                })
            }
            "addi" | "addiu" | "andi" | "ori" | "slti" | "sltiu" =>{
                if tokens.len() < 4{
                    return Err("Cannot parse 2 register, 1 immediate I instruction!".to_string());
                }
                let (rt, rs) = parse_two_regs(&tokens)?;
                let immd = parse_immediate(tokens[3]);
                Ok(match tokens[0] {
                    "addi" => Instr::Addi{rt, rs, immd},
                    "addiu" => Instr::Addiu{rt, rs, immd},
                    "andi" => Instr::Andi{rt, rs, immd},
                    "ori" => Instr::Ori{rt, rs, immd},
                    "slti" => Instr::Slti{rt, rs, immd},
                    "sltiu" => Instr::Sltiu{rt, rs, immd},
                    _=> unreachable!()
                })
            }
            "lw" | "sw" =>{
                if tokens.len() < 4{
                    return Err("Cannot parse load and store instr!".to_string());
                }
                let rt = parse_reg(tokens[1])?;
                let immd = parse_immediate(tokens[2]);
                let rs = parse_reg(tokens[3])?;
                Ok(match tokens[0] {
                    "addi" => Instr::Addi{rt, rs, immd},
                    "addiu" => Instr::Addiu{rt, rs, immd},
                    "andi" => Instr::Andi{rt, rs, immd},
                    "ori" => Instr::Ori{rt, rs, immd},
                    "lw" => Instr::Lw{rt, rs, immd},
                    "sw" => Instr::Sw{rt, rs, immd},
                    "slti" => Instr::Slti{rt, rs, immd},
                    "sltiu" => Instr::Sltiu{rt, rs, immd},
                    _=> unreachable!()
                })
            }
            "mult" | "div" | "move" => {
                if tokens.len() < 3{
                    return Err("Cannot parse 2 register instruction!".to_string());
                }
                let (rs, rt) = parse_two_regs(&tokens)?;
                Ok(match tokens[0] {
                    "mult" => Instr::Mult{rs, rt},
                    "div" => Instr::Div{rs, rt},
                    "move" => Instr::Move{rs, rt},
                    _=> unreachable!()
                })
            }
            "sll" | "srl" => {
                if tokens.len() < 4{
                    return Err("Cannot parse 2 register, 1 shamt shift instruction!".to_string());
                }
                let (rd, rs) = parse_two_regs(&tokens)?;
                let shamt = parse_immediate(tokens[3]);
                Ok(match tokens[0] {
                    "sll" => Instr::Sll{rd, rs, shamt},
                    "srl" => Instr::Srl{rd, rs, shamt},
                    _=> unreachable!()
                })
            }
            "lui" | "li" => {
                if tokens.len() < 3{
                    return Err("Cannot parse 1 register, 1 immediate instruction!".to_string());
                }
                let rt = parse_reg(tokens[1])?;
                let immd = parse_immediate(tokens[2]);
                Ok(match tokens[0] {
                    "lui" => Instr::Lui{rt, immd},
                    "li" => Instr::Li{rt, immd},
                    _=> unreachable!()
                })
            }
            "mfhi" | "mflo" | "jr" => {
                if tokens.len() < 2{
                    return Err("Cannot parse 1 register instruction!".to_string());
                }
                let rd = parse_reg(tokens[1])?;
                Ok(match tokens[0] {
                    "mfhi" => Instr::Mfhi{rd},
                    "mflo" => Instr::Mflo{rd},
                    "jr" => Instr::Jr{rd},
                    _=> unreachable!()
                })
            }
            _ => {return Err(format!("Could Not parse {}",line))}
        }
    }
}
#[derive(Debug)]
pub enum ParsedInstr {
    Label(String),
    I(Instr),
    Empty,
    // Conditional Branch
    Beq{rt: u32, rs: u32, label: String},
    Bne{rt: u32, rs: u32, label: String},
    Bgt{rt: u32, rs: u32, label: String},
    Bge{rt: u32, rs: u32, label: String},
    Blt{rt: u32, rs: u32, label: String},
    Ble{rt: u32, rs: u32, label: String},
    // Unconditional Jump
    Jump{label: String},
    Jal{label: String},
    // Load address
    La{rt: u32, label: String},
}

impl ParsedInstr {
    pub fn from_str(line: &String) -> Result<ParsedInstr, String> {
        let tokens: Vec<&str> = regex::Regex::new(r"([\s()$,]+|#.*)").unwrap()
        .split(line).filter(|s| !s.is_empty()).collect();
        let parse_two_regs = |tokens: &[&str]| -> Result<(u32, u32), String> {
            Ok((parse_reg(tokens[1])?, parse_reg(tokens[2])?))
        };
        // Skip blank lines
        if tokens.len() == 0{
            return Ok(ParsedInstr::Empty);
        }
        match tokens[0]{
            "beq" | "bne" | "bgt" | "bge" | "blt" | "ble" =>{
                if tokens.len() < 4{
                    return Err("Cannot parse branch instruction!".to_string());
                }
                let (rt, rs) = parse_two_regs(&tokens)?;
                Ok(match tokens[0] {
                    "beq" => ParsedInstr::Beq{rt, rs, label: tokens[3].to_string()},
                    "bne" => ParsedInstr::Bne{rt, rs, label: tokens[3].to_string()},
                    "bgt" => ParsedInstr::Bgt{rt, rs, label: tokens[3].to_string()},
                    "bge" => ParsedInstr::Bge{rt, rs, label: tokens[3].to_string()},
                    "blt" => ParsedInstr::Blt{rt, rs, label: tokens[3].to_string()},
                    "ble" => ParsedInstr::Ble{rt, rs, label: tokens[3].to_string()},
                    _=> unreachable!()
                })
            }
            "j" | "jal" => {
                Ok(match tokens[0] {
                    "j" => ParsedInstr::Jump{label: tokens[1].to_string()},
                    "jal" => ParsedInstr::Jal{label: tokens[1].to_string()},
                    _=> unreachable!()
                })
            }
            "la" => {
                if tokens.len() < 3{
                    return Err("Cannot parse LA instruction!".to_string());
                }
                let rt = parse_reg(tokens[1])?;
                Ok(match tokens[0] {
                    "la" => ParsedInstr::La{rt, label: tokens[2].to_string()},
                    _=> unreachable!()
                })
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
