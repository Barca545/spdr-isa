use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::fmt::Display;

// Refactor:
// - Should MemCpy also take offsets?

#[derive(FromPrimitive, ToPrimitive, Debug,)]
/// Unless otherwise stated all immediate arguments are 4 bytes.
pub enum OpCode {
  /// # Halt program execution
  Hlt,
  /// # Load Data
  ///
  /// Load the immediate `I0` into register `R0`.
  ///
  /// Format: `LOAD Rd I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination register.
  /// - `I0`: Source immediate.
  Load,
  /// # Copy Memory
  ///
  /// Copy the value in `R0` into `Rd`.
  ///
  /// Format: COPY Rd R0
  ///
  /// Arguments:
  /// - `Rd`: Destination register.
  /// - `R0`: Source register.
  Copy,
  /// # Memory Copy
  ///
  /// Writes the value stored in the memory address stored in `Rd` into the
  /// memory address stored in `R0`.
  ///
  /// Format:`MEMCPY Rd R0`
  ///
  /// Arguments:
  /// - `Rd`: Destination memory address.
  /// - `R0`: Source memory address.
  MemCpy,
  /// # Add Register and Immediate
  ///
  /// Format: `ADD Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  AddRI,
  /// # Subtract Immediate from Register  
  ///
  /// Format: `SUB Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  SubRI,
  /// # Subtract Register from Immediate
  ///
  /// Format: `RVSUB Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  RvSubRI,
  /// # Multiply Register and Immediate
  ///
  /// Format: `MUL Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  MulRI,
  /// # Divide Register by Immediate
  ///
  /// Format: `DIV Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  DivRI,
  /// # Divide Immediate by Register
  ///
  /// Format: `RVDIV Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  RvDivRI,
  /// # Raise Register by Immediate
  ///
  /// Format: `POW Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  PowRI,
  /// # Raise Immediate by Register
  ///
  /// Format: `RVPOW Rd R0 I0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  RvPowRI,
  /// # Add Register and Register
  ///
  /// Format: `ADD Rd R0 R1`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `R1`: Register operand.
  AddRR,
  /// # Subtract Register and Register
  ///
  /// Format: `SUB Rd R0 R2`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `R1`: Immediate operand.
  SubRR,
  /// # Multiply Register and Register
  ///
  /// Format: `MUL Rd R0 R1`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `R1: Register operand.
  MulRR,
  /// # Divide Register and Register
  ///
  /// Format: `DIV Rd R0 R1`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Register operand.
  /// - `R1`: Register operand.
  DivRR,
  /// # Raise Register by Register
  ///
  /// Format: `POW Rd R0 R1`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: Memory operand.
  /// - `R1`: Register operand.
  PowRR,
  /// # Compare Register and Immediate
  /// Checks whether two values are equal and stores the result in
  /// [`REQ`](crate::registers::EQ).
  ///
  /// Format: `Cmp Fl R0 I0`
  ///
  /// Arguments:
  /// - `Fl`: Flag indicating which comparison operation to perform.
  /// - `R0`: Register operand.
  /// - `I0`: Immediate operand.
  CmpRI,
  /// # Compare Register and Register
  /// Checks whether two values are equal and stores the result in
  /// [`REQ`](crate::registers::EQ).
  ///
  /// Format: `Cmp Fl R0 R1`
  ///
  /// Arguments:
  /// - `Fl`: Flag indicating which comparison operation to perform.
  /// - `R0`: Register operand.
  /// - `R1`: Register operand.
  CmpRR,
  /// # Bitwise Not
  ///
  /// Format:`NOT Rd R0`
  ///
  /// Arguments:
  /// - `Rd`: Destination.
  /// - `R0`: value being negated.
  Not,
  /// # Unconditional Jump
  ///
  /// Format: JMP Idx
  ///
  /// Arguments:
  /// - `Idx`: Destination program index as a `u32`
  Jmp,
  /// # Jump if Zero
  ///
  /// Format: `JZ R0 IDX`
  ///
  /// Arguments:
  /// - `R0`: Register holding the check.
  /// - `Idx`: Destination program index as a `u32`.
  Jz,
  /// # Jump if Not Zero
  ///
  /// Format: `JNZ R0 IDX`
  ///
  /// Arguments:
  /// - `R0`: Register holding the check.
  /// - `Idx`: Destination program index as a `u32`.
  Jnz,
  /// # Call a Function
  ///
  /// Format: `CALL IDX`
  ///
  /// Arguments:
  /// - `Idx`: Location of the function pointer as a `u8`.
  Call,
  /// # System call
  ///
  /// Call an external function.
  ///
  /// Format: `SYSCALL Idx`
  ///
  /// Arguments:
  /// - `Idx`: Index of the external function being called as a `u8`.
  SysCall,
  /// # Return from a function call
  ///
  /// Pop the return address of the top of the stack and set the PC equal to it.
  /// Pop the function's arguments from the stack.
  ///
  /// Format: `RET Idx`
  ///
  /// Arguments:
  /// - `Idx`: The number of function arguments to clean up as a u8.
  // Ret takes a u8 because the stack is only 20 cells long so it will never fill up even a u8 let alone a
  // u32.
  Ret,
  /// # Allocate Memory
  ///
  /// Allocates a slab of memory capable of holding `R0` values. Returns a
  /// pointer to the allocation to `Rd`.
  ///
  /// Format: `ALLOC Rd R0`
  ///
  /// Arguments:
  /// - `Rd`: Register storing the destination.
  /// - `R0`: Register storing the number of values to store.
  Alloc,
  /// # Reallocate Memory
  ///
  /// Reallocates a slab of memory.
  ///
  /// Format: `REALLOC Rd R0`
  ///
  /// Arguments:
  /// - `Rd`: Register storing the previous allocation.
  /// - `R0`: Register storing the number of values to store.
  Realloc,
  /// # Deallocate Memory
  ///
  /// Deallocates a slab of memory.
  ///
  /// Format: `DEALLOC R0`
  ///
  /// Arguments:
  /// - `R0`: Register storing the slab to deallocate.
  Dealloc,
  /// # Read Memory
  ///
  /// Loads the value stored at the pointer in `R0 + I0 + R1` into `Rd`.
  ///
  /// Format:`RMEM Rd R0 I0 R1`
  ///
  /// Arguments:
  /// - `Rd`: Destination
  /// - `R0`: Register storing the source memory address.
  /// - `I0`: Offset stored as an immediate as a u32.
  /// - `R1`: Offset stored in a register.
  ///
  /// Note: If there is no register offset, R2 will be zero and ignored. Zero
  /// (REQ) is used because it will never store an offset.
  RMem,
  /// # Write Memory
  ///
  /// Writes the value stored in `R0 + I0 + R1` into the memory address stored
  /// in `Rd`.
  ///
  /// Format:`RMEM Rd R0 I0 R1`
  ///
  /// Arguments:
  /// - `Rd`: Register storing the destination memory address.
  /// - `R0`: Register storing the data to write to memory.
  /// - `I0`: Offset stored as an immediate as a u32.
  /// - `R1`: Offset stored in a register.
  ///
  /// Note: If there is no register offset, R1 will be zero and ignored. R1 == 0
  /// (the PC register) is used because it will never store an offset.
  WMem,
  /// # Read String
  ///
  /// Given a pointer and a len prints a string into the VM's `stdout` (usually
  /// the terminal).
  ///
  /// Format: `READSTR R0 R1`
  ///
  /// Arguments:
  /// - `R0`: Register storing the pointer to the string's start.
  /// - `R1`: Register storing the string's length.
  WriteStr,
  /// # Push to Stack
  ///
  /// Pushes the argument onto the top of stack.
  ///
  /// Format: `PUSH R0`
  ///
  /// Arguments:
  /// - `R0`: Register holding the value to push.
  Push,
  /// # Pop From Stack
  ///
  /// Format: Takes no args
  ///
  /// Removes the item on the top of the stack.
  Pop,
  /// # Pop Read From Stack
  ///
  /// Removes the item on the top of the stack and places it into a register.
  ///
  /// Format: `POPR R0`
  ///
  /// Arguments:
  /// `R0`: The register to place the popped value.
  PopR,
  /// # No Operation
  Noop,
}

impl From<OpCode,> for u8 {
  fn from(value:OpCode,) -> Self {
    value as u8
  }
}

impl From<u8,> for OpCode {
  fn from(value:u8,) -> Self {
    match FromPrimitive::from_u8(value,) {
      Some(op,) => op,
      None => panic!("{} is not a valid OpCode", value),
    }
  }
}

impl Display for OpCode {
  fn fmt(&self, f:&mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
    match self {
      OpCode::Hlt => write!(f, "Hlt"),
      OpCode::Load => write!(f, "Load"),
      OpCode::AddRI => write!(f, "Add_RI"),
      OpCode::SubRI => write!(f, "Sub_RI"),
      OpCode::RvSubRI => write!(f, "RvSub_RI"),
      OpCode::MulRI => write!(f, "Mul_RI"),
      OpCode::DivRI => write!(f, "Div_RI"),
      OpCode::RvDivRI => write!(f, "RvDiv_RI"),
      OpCode::PowRI => write!(f, "Pow_RI"),
      OpCode::RvPowRI => write!(f, "RvPow_RI"),
      OpCode::AddRR => write!(f, "Add_RR"),
      OpCode::SubRR => write!(f, "Sub_RR"),
      OpCode::MulRR => write!(f, "Mul_RR"),
      OpCode::DivRR => write!(f, "Div_RR"),
      OpCode::PowRR => write!(f, "Pow_RR"),
      OpCode::Call => write!(f, "Call"),
      OpCode::Jz => write!(f, "Jz"),
      OpCode::Jnz => write!(f, "Jnz"),
      OpCode::Jmp => write!(f, "Jmp"),
      OpCode::CmpRI => write!(f, "Cmp_RI"),
      OpCode::CmpRR => write!(f, "Cmp_RR"),
      OpCode::Not => write!(f, "Not"),
      OpCode::Copy => write!(f, "Copy"),
      OpCode::MemCpy => write!(f, "MemCpy"),
      OpCode::SysCall => write!(f, "SysCall"),
      OpCode::Ret => write!(f, "Ret"),
      OpCode::Alloc => write!(f, "Alloc"),
      OpCode::Realloc => write!(f, "Realloc"),
      OpCode::Dealloc => write!(f, "Dealloc"),
      OpCode::RMem => write!(f, "RMem"),
      OpCode::WMem => write!(f, "WMem"),
      OpCode::Push => write!(f, "Push"),
      OpCode::Pop => write!(f, "Pop"),
      OpCode::PopR => write!(f, "PopR"),
      OpCode::Noop => write!(f, "Noop"),
      OpCode::WriteStr => write!(f, "WriteStr"),
    }
  }
}

#[derive(Debug, FromPrimitive,)]
pub enum CmpFlag {
  Eq,
  Gt,
  Lt,
  Geq,
  Leq,
}

impl From<CmpFlag,> for u8 {
  fn from(value:CmpFlag,) -> Self {
    value as u8
  }
}

impl From<u8,> for CmpFlag {
  fn from(value:u8,) -> Self {
    match FromPrimitive::from_u8(value,) {
      Some(op,) => op,
      None => panic!("{} is not a valid Cmpflag", value),
    }
  }
}

impl Display for CmpFlag {
  fn fmt(&self, f:&mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
    match self {
      CmpFlag::Eq => write!(f, "EQ"),
      CmpFlag::Gt => write!(f, "GT"),
      CmpFlag::Lt => write!(f, "LT"),
      CmpFlag::Geq => write!(f, "GEQ"),
      CmpFlag::Leq => write!(f, "LEQ"),
    }
  }
}
