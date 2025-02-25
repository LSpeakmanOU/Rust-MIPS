mod cpu;
use cpu::CPU;
mod isa;
use isa::*;
use std::env;
use std::io::{self, BufRead, BufReader};
use std::io::{Error,ErrorKind};
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    // Get arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        println!("Usage: program <input MIPS script>");
        std::process::exit(-1);
    }
    // Open file for reading
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    // Stores parsed instructions
    let mut p_instrs:Vec<ParsedInstr> = vec![];
    // Stores label mappings
    let mut labels: HashMap<String, u32> = HashMap::new();
    // Stores current instruction number
    let mut instr_number: u32 = 0;
    // For each line
    for line in reader.lines() {
        match line{
            Ok(l) => {
                // Attempt to parse line as ParsedInstr type
                let line_parsed = ParsedInstr::from_str(&l);
                // Skip any NOPs
                match line_parsed{
                    // Skip all empty lines
                    Ok(ParsedInstr::Empty) => {}
                    // Map labels to the current instruction location(in words)
                    Ok(ParsedInstr::Label(s)) => {
                        labels.insert(s, instr_number);
                    }
                    // If its a usual instruction, add the instruction and increment the instruction number
                    Ok(p_instr) => {
                        p_instrs.push(p_instr);
                        instr_number += 1;
                    },
                    Err(s) => {
                        return Err(Error::new(ErrorKind::Other,format!("Could not parse instruction! {}",s)))
                    },
                }
            }
            Err(_) => continue,
        }
    }
    // Map parsed instructions to instructions ready to execute
    let mut instructions: Vec<Instr> = Vec::new();
    let mut instr_number: i32 = 0;
    for inst in p_instrs{
        match &inst{
            // Typical instructions can just be pulled out of any parsed instructions
            ParsedInstr::I(inner) => instructions.push(inner.clone()),
            // Handle labelled instructions
            // Beq, Bne, etc use relative addresses in words
            // J, and Jal use absolute addresses in words
            ParsedInstr::Beq{rt, rs, label} | ParsedInstr::Bne{rt, rs, label} | ParsedInstr::Bgt{rt, rs, label} |
            ParsedInstr::Bge{rt, rs, label} | ParsedInstr::Blt{rt, rs, label} | ParsedInstr::Ble{rt, rs, label} => {
                let addr: u32 = labels.get(label).unwrap().clone();
                let rel_addr: i32 = (addr as i32 - (instr_number as i32 + 1)).try_into().unwrap();
                let instr = match &inst {
                    ParsedInstr::Beq{..} => Instr::Beq{rt: *rt, rs: *rs, rel_addr},
                    ParsedInstr::Bne{..} => Instr::Bne{rt: *rt, rs: *rs, rel_addr},
                    ParsedInstr::Bgt{..} => Instr::Bgt{rt: *rt, rs: *rs, rel_addr},
                    ParsedInstr::Bge{..} => Instr::Bge{rt: *rt, rs: *rs, rel_addr},
                    ParsedInstr::Blt{..} => Instr::Blt{rt: *rt, rs: *rs, rel_addr},
                    ParsedInstr::Ble{..} => Instr::Ble{rt: *rt, rs: *rs, rel_addr},
                    _ => unreachable!(),
                };
                instructions.push(instr);
            }
            ParsedInstr::Jump{label} | ParsedInstr::Jal{label} => {
                let addr: u32 = labels.get(label).unwrap().clone();
                let instr = match &inst {
                    ParsedInstr::Jump{..} => Instr::Jump{addr},
                    ParsedInstr::Jal{..} => Instr::Jal{addr},
                    _ => unreachable!(),
                };
                instructions.push(instr);
            }
            ParsedInstr::La{rt, label} => {
                let addr: u32 = labels.get(label).unwrap().clone();
                let instr = match &inst {
                    ParsedInstr::La{..} => Instr::La{rt: *rt, addr},
                    _ => unreachable!(),
                };
                instructions.push(instr);
            }
            // Handle anything like labels or NOPs that slipped through(shouldn't happen)
            _ => {}
        }
        instr_number += 1;
    }
    // Now that we have a set of instructions, execute them
    let mut cpu = CPU::new();
    let debug: bool = true;
    while cpu.pc < instructions.len(){
        // Fetch
        let fetch = instructions[cpu.pc];
        // Decode and Execute
        cpu.execute(&fetch);
        println!("{:?}",fetch);
    }
    
    
    Ok(())
}
