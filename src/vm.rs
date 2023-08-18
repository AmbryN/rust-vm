use crate::{assembler::PIE_HEADER_PREFIX, instruction::Opcode};

pub struct VM {
    // Array simulating hardware registers
    pub registers: [i32; 32],
    // Program counter: which byte is being executed
    pc: usize,
    // Instructions of the program
    pub program: Vec<u8>,
    // Remainder of division operation
    remainder: u32,
    // Result of last comparison
    equal_flag: bool,
    heap: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
            heap: vec![],
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }

    pub fn run(&mut self) {
        self.verify_header();

        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn verify_header(&mut self) -> bool {
        if self.program[0..4] != PIE_HEADER_PREFIX {
            return false;
        }
        self.pc = 65;
        true
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as usize;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register] as usize;
                if self.equal_flag {
                    self.pc = target;
                } else {
                    self.next_16_bits();
                }
            }
            Opcode::JNEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register] as usize;
                if self.equal_flag {
                    self.next_16_bits();
                } else {
                    self.pc = target;
                }
            }
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::INC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            }
            Opcode::DEC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] -= 1;
            }
            _ => {
                panic!("Unrecognized opcode found! Terminating!");
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    pub fn add_bytes(&mut self, mut bytes: Vec<u8>) {
        self.program.append(&mut bytes);
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::assembler::PIE_HEADER_LENGTH;

    use super::*;

    fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
        let mut header = Vec::with_capacity(PIE_HEADER_LENGTH);
        header.append(&mut PIE_HEADER_PREFIX.to_vec());
        header.fill(0);
        header.append(&mut b);
        header
    }

    #[test]
    fn test_createvm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        prepend_header(test_vm.program);
        let test_bytes = vec![11, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    #[should_panic]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        prepend_header(test_vm.program);
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        prepend_header(test_vm.program);
        test_vm.program = vec![0, 0, 1, 244]; // 500 en binaire u16 little endian
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::new();
        let test_bytes = vec![
            0, 0, 0, 10, // LOAD $0 #10 : Charger 10 dans reg 0
            0, 1, 0, 15, // LOAD $1 #15 : Charger 15 dans reg 1
            1, 0, 1, 2, //ADD $0 $1 $2 : Ajouter reg 0 et 1 dans reg 2
        ];
        prepend_header(test_vm.program);
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.registers[2], 25);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::new();
        let test_bytes = vec![
            0, 0, 0, 15, // LOAD $0 #10 : Charger 10 dans reg 0
            0, 1, 0, 10, // LOAD $1 #15 : Charger 15 dans reg 1
            2, 0, 1, 2, // SUB $0 $1 $2 : Soustraire reg 0 et 1 dans reg 2
        ];
        test_vm.program = test_bytes;
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        let test_bytes = vec![
            0, 0, 0, 10, // LOAD $0 #10 : Charger 10 dans reg 0
            0, 1, 0, 15, // LOAD $1 #15 : Charger 15 dans reg 1
            3, 0, 1, 2, // MUL $0 $1 $2 : Multplier reg 0 et 1 dans reg 2
        ];
        test_vm.program = test_bytes;
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 150);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::new();
        let test_bytes = vec![
            0, 0, 0, 20, // LOAD $0 #10 : Charger 10 dans reg 0
            0, 1, 0, 15, // LOAD $1 #15 : Charger 15 dans reg 1
            4, 0, 1, 2, // DIV $0 $1 $2 : Diviser reg 0 et 1 dans reg 2
        ];
        test_vm.program = test_bytes;
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 5);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![5, 0, 0, 0]; // JMP $0 : Saut vers pc = valeur reg 0, donc 1
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0, 0, 0, 10, // LOAD $0 #10 : Charger 10 dans reg 0
            0, 1, 0, 15, // LOAD $1 #15 : Charger 15 dans reg 1
            3, 0, 1, 2, // MUL $0 $1 $2 : Multiplier reg 0 et 1 dans reg 2
            0, 3, 0, 18, // LOAD $3 #18
            7, 3, 0, 0, // JMPB $3
        ];
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![
            8, 0, 1, 0, // EQ $0 $1 : reg 0 est-il égal reg 1
            8, 0, 1, 0, // EQ $0 $1 : reg 0 est-il égal reg 1
        ];
        test_vm.run_once();
        assert!(test_vm.equal_flag);

        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert!(!test_vm.equal_flag);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.equal_flag = true;
        test_vm.program = vec![9, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_jneq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.equal_flag = false;
        test_vm.program = vec![10, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 2);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![12, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_inc_opcode() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 1;
        test_vm.program = vec![13, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 2);
    }

    #[test]
    fn test_dec_opcode() {
        let mut test_vm = VM::default();
        test_vm.registers[0] = 1;
        test_vm.program = vec![14, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 0);
    }
}
