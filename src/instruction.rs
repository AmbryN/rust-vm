#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    EQ,
    JEQ,
    JNEQ,
    IGL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::JMP,
            6 => Opcode::JMPF,
            7 => Opcode::JMPB,
            8 => Opcode::EQ,
            9 => Opcode::JEQ,
            10 => Opcode::JNEQ,
            11 => Opcode::HLT,
            _ => Opcode::IGL,
        }
    }
}

impl From<&str> for Opcode {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "EQ" => Opcode::EQ,
            "JEQ" => Opcode::JEQ,
            "JNEQ" => Opcode::JNEQ,
            "HLT" => Opcode::HLT,
            _ => Opcode::IGL,
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let str = "load";
        assert_eq!(Opcode::LOAD, Opcode::from(str));
        let str = "LOAD";
        assert_eq!(Opcode::LOAD, Opcode::from(str));
        let str = "ADD";
        assert_eq!(Opcode::ADD, Opcode::from(str));
        let str = "illegal";
        assert_eq!(Opcode::IGL, Opcode::from(str));
    }
}
