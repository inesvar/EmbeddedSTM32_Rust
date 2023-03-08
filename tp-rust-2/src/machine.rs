use std::io::{self, Write};


const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

#[derive(Debug)]
pub struct Machine {
    memory : [u8 ; MEMORY_SIZE],
    regs : [u32 ; NREGS],
}

#[derive(Debug)]
pub enum MachineError {
    InvalidRegister(usize),
    InvalidInstruction(u8),
    InvalidMemoryAddress(usize),
    InsufficientPointerSize,
    WriteError,
}

#[derive(Copy, Clone)]
enum InstructionType {
    MoveIf,
    Store,
    Load,
    LoadImm,
    Sub,
    Out,
    Exit,
    OutNumber,
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Panics
    /// This function panics when `memory` is larger than the machine memory.
    pub fn new(memory: &[u8]) -> Self {

        // building the memory
        let mut array_memory : [u8 ; MEMORY_SIZE] = [0 ; MEMORY_SIZE];
        let memory_size :usize = memory.len();
        if memory_size > MEMORY_SIZE {
            panic!("memory is larger than the machine memory");
        }
        array_memory[..memory_size].copy_from_slice(memory);

        let regs : [u32 ; NREGS] = [0 ; NREGS];
        // initializing the registers to example values
        /*let mut regs : [u32 ; NREGS] = [0 ; NREGS];
        regs[1] = 10;
        regs[2] = 25;
        regs[3] = 0x1234ABCD;
        regs[4] = 0;
        regs[5] = 65;*/

        println!("\nCreating a virtual machine...\nmemory : {array_memory:?}, regs : {regs:?}");
        Machine{memory : array_memory, regs}
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on `fd`.
    pub fn run_on<T: Write>(&mut self, fd: &mut T) -> Result<(), MachineError> {
        while !self.step_on(fd)? {}
        Ok(())
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on standard output.
    pub fn run(&mut self) -> Result<(), MachineError> {
        self.run_on(&mut io::stdout().lock())
    }

    /// Execute the next instruction by doing the following steps:
    ///   - decode the instruction located at IP (register 0)
    ///   - increment the IP by the size of the instruction
    ///   - execute the decoded instruction
    ///
    /// If output instructions are run, they print on `fd`.
    /// If an error happens at either of those steps, an error is
    /// returned.
    ///
    /// In case of success, `true` is returned if the program is
    /// terminated (upon encountering an exit instruction), or
    /// `false` if the execution must continue.
    pub fn step_on<T: Write>(&mut self, fd: &mut T) -> Result<bool, MachineError> {
        
        // getting the length of the instruction
        let _instruction_type :InstructionType = self.instruction_type()?;
        let _instruction_length :usize = Machine::instruction_length(_instruction_type);

        // incrementing IP and fetching the instruction
        let pc :usize = self.get_reg(IP)? as usize;
        let instruction :&[u8];
        let new_pc :Option<usize> = pc.checked_add(_instruction_length);
        match new_pc {
            Some(n) if n <= MEMORY_SIZE => {self.set_reg(IP, n as u32)?; instruction = &self.memory[pc..new_pc.unwrap()]},
            Some(n) =>  {self.set_reg(IP, MEMORY_SIZE as u32)?; return Err(MachineError::InvalidMemoryAddress(n));},
            None => {return Err(MachineError::InsufficientPointerSize);},
        }

        match _instruction_type {
            InstructionType::MoveIf => { // MOVE IF : regA (1) = regB (2) if regC (3) != 0
                let reg_c = self.get_reg(instruction[3] as usize)?;
                if reg_c != 0 { // if regC != 0
                    let reg_b = self.get_reg(instruction[2] as usize)?;
                    self.set_reg(instruction[1] as usize, reg_b)?; // regA = regB
                }
                Ok(false)
                },
            InstructionType::Store => { // STORE : *regA (1) = regB (2)
                let address :usize = self.get_reg(instruction[1] as usize)? as usize;
                if address + 3 > MEMORY_SIZE - 1 {
                    return Err(MachineError::InvalidMemoryAddress(address + 3));
                }
                let reg_b = self.get_reg(instruction[2] as usize)?;
                for i in 0..4 {
                    self.memory[address + i] = reg_b.to_le_bytes()[i];
                }
                Ok(false)
                },
            InstructionType::Load => { // LOAD : regA (1) = *regB (2)
                let address :usize = self.get_reg(instruction[2] as usize)? as usize;
                let mut reg_a :u32 = 0;
                for i in 0..4 {
                    reg_a += (self.load_from_memory(address + i)? as u32) << (8 * i);
                }
                self.set_reg(instruction[1] as usize, reg_a)?;
                Ok(false)
                },
            InstructionType::LoadImm => { // LOADIMM : regA (1) = (H (3) << 8) | L (2) as i32
                let imm :u16 = ((instruction[3] as u16) << 8) | (instruction[2] as u16);
                let reg_a :u32 = if imm.leading_zeros() > 0 { // if imm is negative
                    imm as u32
                } else { // if imm is negative
                    (imm as u32) | 0xFFFF0000
                };
                self.set_reg(instruction[1] as usize, reg_a)?;
                Ok(false)
                },
            InstructionType::Sub => { // SUB : regA (1) = regB (2) - regC (3)
                let substraction : u32 = self.get_reg(instruction[2] as usize)?.wrapping_sub(self.get_reg(instruction[3] as usize)?);
                self.set_reg(instruction[1] as usize, substraction)?;
                Ok(false)
                },
            InstructionType::Out => { // OUT : print low 8 bits of regA (1) on fd
                let reg_a = self.get_reg(instruction[1] as usize)?;
                match write!(fd, "{}", (reg_a & 0xFF) as u8 as char) {
                    Ok(_) => (),
                    Err(_) => return Err(MachineError::WriteError),
                }
                Ok(false)
                },
            InstructionType::Exit => {
                println!("\nExiting the program...\nmemory : {:?}, regs : {:?}", self.memory, self.regs);
                Ok(true)
                },
            InstructionType::OutNumber => { // OUTNUMBER : print the signed number in regA (1) in decimal on fd
                let reg_a = self.get_reg(instruction[1] as usize)?;
                match write!(fd, "{}", reg_a as i32) {
                    Ok(_) => (),
                    Err(_) => return Err(MachineError::WriteError),
                }
                Ok(false)
                },
        }
    }

    /// Similar to [step_on](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool, MachineError> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    pub fn regs(&self) -> &[u32] {
        &self.regs[..]
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<(), MachineError> {
        match reg {
            n if n < 16 => Ok(self.regs[reg] = value),
            _ =>  Err(MachineError::InvalidRegister(reg)),
        }
    }

    /// Gets the value of a given register.
    fn get_reg(&self, reg: usize) -> Result<u32, MachineError> {
        match reg {
            n if n < 16 => Ok(self.regs[reg]),
            _ =>  Err(MachineError::InvalidRegister(reg)),
        }
    }

    /// Reference onto the machine current memory.
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Gets a u8 value from a given place in the memory
    fn load_from_memory(&self, address :usize) -> Result<u8, MachineError> {
        match address {
            n if n < MEMORY_SIZE => Ok(self.memory[address]),
            n => Err(MachineError::InvalidMemoryAddress(n)),
        }
    }

    /// Returns the type of the instruction which starts at the address pointed by the IP register
    fn instruction_type(&self) -> Result<InstructionType, MachineError> {
        let _instruction_type = self.load_from_memory(self.regs[IP] as usize)?;
        match _instruction_type {
            1 => Ok(InstructionType::MoveIf),
            2 => Ok(InstructionType::Store),
            3 => Ok(InstructionType::Load),
            4 => Ok(InstructionType::LoadImm),
            5 => Ok(InstructionType::Sub),
            6 => Ok(InstructionType::Out),
            7 => Ok(InstructionType::Exit),
            8 => Ok(InstructionType::OutNumber),
            _ => Err(MachineError::InvalidInstruction(_instruction_type)),
        }
    }

    /// Returns the length of the instruction which type is given
    fn instruction_length(instruction_type :InstructionType) -> usize {
        match instruction_type {
            InstructionType::MoveIf => 4,
            InstructionType::Store => 3,
            InstructionType::Load => 3,
            InstructionType::LoadImm => 4,
            InstructionType::Sub => 4,
            InstructionType::Out => 2,
            InstructionType::Exit => 1,
            InstructionType::OutNumber => 2,
        }
    }
}
