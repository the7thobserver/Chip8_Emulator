pub struct Cpu {
	id: u32,
	extra: u32
}

impl Cpu {
	pub fn new() -> Cpu { 
		Cpu {
			// Setup default inti
			id: 1,
			extra: 1,
		}
	}
	
	pub fn isAlive(&self) {
		println!("I'M ALIVE");
	}
	
	fn execute_instruction(&mut self, instruction) {
		match instruction {
			0x00 => {
			
			}
			0x01 => {}
			0x02 => {}
			0x03 => {}
			0x04 => {}
			0x05 => {}
			0x06 => {}
			0x07 => {}
			0x08 => {}
			0x09 => {}
			0x0a => {}
			0x0b => {}
			0x0c => {}
			0x0d => {}
			0x0e => {}
			0x0f => {}
			
		}
	}
}
