/// Length of the [`VM`](https://github.com/Barca545/galaxy)'s memory array.
pub const MEM_SIZE:usize = u16::MAX as usize;

/// Length of the portion of the [`VM`](https://github.com/Barca545/galaxy)'s memory array used as the
/// "stack". Valid addresses are mem0-mem19.
pub const STACK_SIZE:usize = 20;
