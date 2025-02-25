pub struct CPU{
    pub pc:u32,
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
}