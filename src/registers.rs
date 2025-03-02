/// Number of registers in the [`VM`](https://github.com/Barca545/galaxy).
pub const REG_NUMBER:u8 = u8::MAX;

/// Program counter. Contains the address of the next
/// [`OpCode`](crate::opcodes::OpCode) instruction.
pub const PC:usize = 0;

/// Stack pointer. Points to the top (last filled) slot on the stack.
pub const SP:usize = 1;

/// Register which holds the result of the [`VM`](https://github.com/Barca545/galaxy)'s last equality check.
pub const EQ:usize = 2;
