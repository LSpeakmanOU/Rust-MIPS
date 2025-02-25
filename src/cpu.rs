use crate::isa::Instr;

pub struct CPU{
    pub pc:usize,
    pub hi:u32,
    pub lo:u32,
    pub reg:[u32;32],
    pub delay_slot_ready: bool,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            pc:0,
            hi:0,
            lo:0,
            reg:[0; 32],
            delay_slot_ready:false,
        }
    }
    pub fn get_reg(&self, index: u32) -> Result<u32, String> {
        if index < self.reg.len() as u32 {
            return Ok(self.reg[index as usize]);
        } else{
            return Err("Out of bounds register get".to_string());
        }
    }

    pub fn set_reg(&mut self, index: u32, value: u32) -> Result<(), String> {
        if index < self.reg.len() as u32 {
            if index == 0 {
                return Err("Invalid setting of $0 register".to_string());
            }
            self.reg[index as usize] = value;
            return Ok(());
        } else {
            return Err("Out of bounds register set".to_string());
        }
    }
    pub fn execute(&mut self, instr : &Instr) -> Result<(), String>{
        match instr{
            Instr::Add{rd, rs, rt} => {
                self.set_reg(*rd, self.get_reg(*rs)?+self.get_reg(*rt)?)
            }
            Instr::Sub{rd, rs, rt} => {
                self.set_reg(*rd, self.get_reg(*rs)?-self.get_reg(*rt)?)
            }
            Instr::Addu{rd, rs, rt} => {
                Ok(())
            }
            Instr::Subu{rd, rs, rt} => {Ok(())}
            Instr::Addi{rt, rs, immd} => {Ok(())}
            Instr::Addiu{rt, rs, immd} => {Ok(())}
            Instr::Mul{rd, rs, rt} => {Ok(())}
            Instr::Mult{rs, rt} => {Ok(())}
            Instr::Div{rs, rt} => {Ok(())}
            Instr::And{rd, rs, rt} => {Ok(())}
            Instr::Or{rd, rs, rt} => {Ok(())}
            Instr::Andi{rt, rs, immd} => {Ok(())}
            Instr::Ori{rt, rs, immd} => {Ok(())}
            Instr::Sll{rd, rs, shamt} => {Ok(())}
            Instr::Srl{rd, rs, shamt} => {Ok(())}
            Instr::Lw{rt, rs, immd} => {Ok(())}
            Instr::Sw{rt, rs, immd} => {Ok(())}
            Instr::Lui{rt, immd} => {Ok(())}
            Instr::La{rt, addr} => {Ok(())}
            Instr::Li{rt, immd} => {Ok(())}
            Instr::Mfhi{rd} => {Ok(())}
            Instr::Mflo{rd} => {Ok(())}
            Instr::Move{rs, rt} => {Ok(())}
            Instr::Beq{rt, rs, rel_addr} => {Ok(())}
            Instr::Bne{rt, rs, rel_addr} => {Ok(())}
            Instr::Bgt{rt, rs, rel_addr} => {Ok(())}
            Instr::Bge{rt, rs, rel_addr} => {Ok(())}
            Instr::Blt{rt, rs, rel_addr} => {Ok(())}
            Instr::Ble{rt, rs, rel_addr} => {Ok(())}
            Instr::Slt{rd, rs, rt} => {Ok(())}
            Instr::Slti{rt, rs, immd} => {Ok(())}
            Instr::Sltiu{rt, rs, immd} => {Ok(())}
            Instr::Jump{addr} => {
                self.pc = *addr as usize;
                self.delay_slot_ready = true;
                Ok(())
            }
            Instr::Jr{rd} => {Ok(())}
            Instr::Jal{addr} => {
                self.set_reg(31, self.pc as u32+8)?;
                self.pc = *addr as usize;
                self.delay_slot_ready = true;
                Ok(())
            }
        }
    }
}
