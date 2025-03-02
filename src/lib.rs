#![feature(iter_next_chunk)]
pub mod memory;
mod opcodes;
pub mod program;
pub mod registers;

pub use opcodes::OpCode;
