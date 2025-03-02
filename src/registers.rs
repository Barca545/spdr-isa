/// Number of registers in the [`VM`](https://github.com/Barca545/galaxy).
pub const REG_COUNT:usize = u8::MAX as usize;

/// First non-reserved register in the [`VM`](https://github.com/Barca545/galaxy). Registers R4-R13 are reserved for function arguments.
pub const FIRST_FREE_REGISTER:usize = 13;

/// Program counter. Contains the address of the next
/// [`OpCode`](crate::opcodes::OpCode) instruction.
pub const PC:usize = 0;

/// Stack pointer. Points to the top (last filled) slot on the stack.
pub const SP:usize = 1;

/// Register which holds the result of the [`VM`](https://github.com/Barca545/galaxy)'s last equality check.
pub const EQ:usize = 2;

/// Register which holds the [loop variable](https://en.wikipedia.org/wiki/For_loop) of the [`VM`](https://github.com/Barca545/galaxy)'s currently executing loop
pub const LOOP:usize = 3;
