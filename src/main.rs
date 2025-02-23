mod cpu;
use cpu::CPU;
mod isa;
use isa::*;
use std::env;
use std::io::{self, BufRead, BufReader, Write};
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
    // Stores current instruction number(TODO Change behavior when assembler is implemented)
    let mut instr_number: u32 = 0;
    // For each line
    for line in reader.lines() {
        match line{
            Ok(l) => {
                // Attempt to parse line as ParsedInstr type
                let line_parsed = ParsedInstr::from_str(&l);
                // Skip any NOPs
                match line_parsed{
                    // Skip all NOPs(TODO change to separate NOP from the idea of no instruction as these are different)
                    Ok(ParsedInstr::NOP) => {}
                    // Map labels to the current instruction location
                    Ok(ParsedInstr::Label(s)) => {
                        labels.insert(s, instr_number);
                    }
                    // If its a usual instruction, add the instruction and increment the instruction number
                    Ok(p_instr) => {
                        p_instrs.push(p_instr);
                        instr_number += 1;
                    },
                    Err(_) => return Err(Error::new(ErrorKind::Other,format!("Could not parse instruction! {}",l))),
                }
            }
            Err(_) => continue,
        }
    }
    // Map parsed instructions to instructions ready to execute
    let mut instructions: Vec<Instr> = Vec::new();
    for inst in p_instrs{
        match inst{
            // Typical instructions can just be pulled out of any parsed instructions
            ParsedInstr::I(inner) => instructions.push(inner),
            // Handle labelled instructions
            ParsedInstr::Beq(r1, r2, label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Beq(r1,r2,*addr));
            },
            ParsedInstr::Bne(r1, r2, label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Bne(r1,r2,*addr));
            },
            ParsedInstr::Bgt(r1, r2, label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Bgt(r1,r2,*addr));
            },
            ParsedInstr::Bge(r1, r2, label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Bge(r1,r2,*addr));
            },
            ParsedInstr::Blt(r1, r2, label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Blt(r1,r2,*addr));
            },
            ParsedInstr::Ble(r1, r2, label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Ble(r1,r2,*addr));
            },
            ParsedInstr::Jump(label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Jump(*addr));
            },
            ParsedInstr::Jr(label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Jr(*addr));
            },
            ParsedInstr::Jal(label) => {
                let addr: &u32 = labels.get(&label).expect("Cant get address");
                instructions.push(Instr::Jal(*addr));
            },
            // Handle anything like labels or NOPs that slipped through(shouldn't happen)
            _ => {}
        }
    }
    // Now that we have a set of instructions, execute them
    let mut cpu = CPU::new();
    for ins in instructions{
        println!("{:?}",ins);
    }
    
    
    Ok(())
}
