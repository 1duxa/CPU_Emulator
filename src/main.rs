#[derive(Debug)]
struct Memory {
    data: [u8; 1024 * 64],
}

impl Memory {
    const MAX_MEM: u32 = 1024 * 64;
    pub fn new() -> Memory {
        Self {
            data: [0; Memory::MAX_MEM as usize],
        }
    }
    pub fn reset(&mut self) {
        self.data = [0; Memory::MAX_MEM as usize];
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default)]
struct CPU {
    program_counter: u16,
    stack_register: u16,
    // registers
    A: u8,
    X: u8,
    Y: u8,
    processor_status: u8,
    C: u8, // Carry flag
    Z: u8, // Zero flag
    I: u8, // Interrupt Disable
    D: u8, // Decimal mode
    B: u8, // Break Command
    O: u8, // Overflow flag
    N: u8, // Negative flag
}

impl CPU {
    pub fn new() -> CPU {
        Self {
            ..Default::default()
        }
    }
    pub fn reset(&mut self, memory: &mut Memory) {
        self.program_counter = 0xFFFC;
        self.stack_register = 0x00FF;

        self.A = 0;
        self.X = 0;
        self.Y = 0;

        self.C = 0;
        self.Z = 0;
        self.I = 0;
        self.D = 0;
        self.B = 0;
        self.O = 0;
        self.N = 0;

        memory.reset();
    }
    pub fn lda_set_status(&mut self) {
        self.Z = (self.A == 0) as u8;
        self.N = ((self.A & 0b10000000) > 0) as u8;
    }
    pub fn fetch(&mut self, memory: &Memory) -> u8 {
        let data = memory.data[self.program_counter as usize];
        self.program_counter += 1;
        data
    }
    pub fn read_byte(&mut self, byte: u8, memory: &Memory) -> u8 {
        let data = memory.data[byte as usize];
        self.program_counter += 1;
        data
    }

    pub fn exec(&mut self, mut ticks: u32, memory: &Memory) {
        while ticks > 0 {
            let ins = self.fetch(memory); // 1 cycle
            ticks -= 1;
            match ins {
                CPU::INS_LDA_IM => {
                    let value = self.fetch(memory); // 1 cycle
                    ticks -= 1;
                    self.A = value;
                    self.lda_set_status();
                    println!("{:#?}", self);
                }
                CPU::INS_LDA_ZP => {
                    let zero_page_addr = self.fetch(memory); // 1 cycle
                    ticks -= 1;
                    self.A = self.read_byte(zero_page_addr, memory);
                    ticks -= 1;
                    self.lda_set_status();
                }
                _ => {
                    println!("Miss!");
                    break;
                }
            }
        }
    }
    const INS_LDA_IM: u8 = 0xA9;
    const INS_LDA_ZP: u8 = 0xA5;
}

fn main() {
    let mut memory = Memory::new();
    let mut cpu: CPU = CPU::new();
    cpu.reset(&mut memory);
    memory.data[0xFFFC] = CPU::INS_LDA_IM; // Opcode for LDA #immediate
    memory.data[0xFFFD] = 0x42; // Immediate value (42 in decimal)
    cpu.exec(2, &memory); // We start with 2 ticks, one for opcode, one for operand
}
