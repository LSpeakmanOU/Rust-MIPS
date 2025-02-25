use crate::isa::Instr;

pub struct CPU{
    pub pc:usize,
    pub hi:u32,
    pub lo:u32,
    pub reg:[u32;32],
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc:0,
            hi:0,
            lo:0,
            reg:[0; 32]
        }
    }
    pub fn get_reg(&self, index: usize) -> Result<u32, String> {
        if index < self.reg.len() {
            return Ok(self.reg[index]);
        } else{
            return Err("Out of bounds register get".to_string());
        }
    }

    pub fn set_reg(&mut self, index: usize, value: u32) -> Result<(), String> {
        if index < self.reg.len() {
            if index == 0 {
                return Err("Invalid setting of $0 register".to_string());
            }
            self.reg[index] = value;
            return Ok(());
        } else {
            return Err("Out of bounds register set".to_string());
        }
    }
    pub fn execute(&mut self, instr : &Instr){
        match instr{
            Instr::Add{rd, rs, rt} => {}
            Instr::Sub{rd, rs, rt} => {}
            Instr::Addu{rd, rs, rt} => {}
            Instr::Subu{rd, rs, rt} => {}
            Instr::Addi{rt, rs, immd} => {}
            Instr::Addiu{rt, rs, immd} => {}
            Instr::Mul{rd, rs, rt} => {}
            Instr::Mult{rs, rt} => {}
            Instr::Div{rs, rt} => {}
            Instr::And{rd, rs, rt} => {}
            Instr::Or{rd, rs, rt} => {}
            Instr::Andi{rt, rs, immd} => {}
            Instr::Ori{rt, rs, immd} => {}
            Instr::Sll{rd, rs, shamt} => {}
            Instr::Srl{rd, rs, shamt} => {}
            Instr::Lw{rt, rs, immd} => {}
            Instr::Sw{rt, rs, immd} => {}
            Instr::Lui{rt, immd} => {}
            Instr::La{rt, addr} => {}
            Instr::Li{rt, immd} => {}
            Instr::Mfhi{rd} => {}
            Instr::Mflo{rd} => {}
            Instr::Move{rs, rt} => {}
            Instr::Beq{rt, rs, rel_addr} => {}
            Instr::Bne{rt, rs, rel_addr} => {}
            Instr::Bgt{rt, rs, rel_addr} => {}
            Instr::Bge{rt, rs, rel_addr} => {}
            Instr::Blt{rt, rs, rel_addr} => {}
            Instr::Ble{rt, rs, rel_addr} => {}
            Instr::Slt{rd, rs, rt} => {}
            Instr::Slti{rt, rs, immd} => {}
            Instr::Sltiu{rt, rs, immd} => {}
            Instr::Jump{addr} => {}
            Instr::Jr{rd} => {}
            Instr::Jal{addr} => {}
        }
    }
}
