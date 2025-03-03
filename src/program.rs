use crate::opcodes::{CmpFlag, OpCode};
use eyre::Result;
use std::{
  fmt::{Debug, Display},
  fs::File,
  io::{Read, Write},
  mem::transmute,
  ops::{Index, IndexMut},
};

// Refactor:
// - Add a thing so a target can only be updated once?
// - Add better errors to the save/load functions

#[derive(Clone,)]
/// A VM program.
///
/// - `Program` is indexed with [`u32`] so every index into it is `[u8;4]`.
pub struct Program {
  inner:Vec<u8,>,
}

impl<const N: usize,> From<&[u8; N],> for Program {
  fn from(value:&[u8; N],) -> Self {
    Program {
      inner:Vec::from(value,),
    }
  }
}

impl From<Vec<u8,>,> for Program {
  fn from(value:Vec<u8,>,) -> Self {
    Program { inner:value, }
  }
}

impl Index<u32,> for Program {
  type Output = u8;

  fn index(&self, index:u32,) -> &Self::Output {
    &self.inner[index as usize]
  }
}

impl IndexMut<u32,> for Program {
  fn index_mut(&mut self, index:u32,) -> &mut Self::Output {
    &mut self.inner[index as usize]
  }
}

impl Display for Program {
  fn fmt(&self, f:&mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
    let mut output = Vec::new();
    let mut src = self.inner.clone().into_iter();

    while let Some(val,) = src.next() {
      let op = OpCode::from(val,);

      match op {
        OpCode::Hlt => output.push(op.to_string(),),
        OpCode::Load => {
          let target = src.next().unwrap();
          let num = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push(format!("{} ${}, {}", op, target, num),);
        }
        OpCode::AddRI
        | OpCode::SubRI
        | OpCode::MulRI
        | OpCode::DivRI
        | OpCode::PowRI
        | OpCode::RvSubRI
        | OpCode::RvDivRI
        | OpCode::RvPowRI => {
          let target = src.next().unwrap();
          let a = src.next().unwrap();
          let b = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push(format!("{} ${}, ${} {}", op, target, a, b),);
        }
        OpCode::AddRR | OpCode::SubRR | OpCode::MulRR | OpCode::DivRR | OpCode::PowRR => {
          let target = src.next().unwrap();
          let a = src.next().unwrap();
          let b = src.next().unwrap();
          output.push(format!("{} ${}, ${} ${}", op, target, a, b),);
        }
        OpCode::Jmp => {
          let ln = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push(format!("{}, {}", op, ln),);
        }
        OpCode::Jnz | OpCode::Jz => {
          let idx = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          let cond = src.next().unwrap();
          output.push(format!("{}, {}, ${}", op, idx, cond),);
        }
        OpCode::CmpRI => {
          let fl = CmpFlag::from(src.next().unwrap(),);
          let a = src.next().unwrap();
          let b = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          // dbg!(b);
          output.push(format!("{}, {}, ${}, {}", op, fl, a, b),);
        }
        OpCode::CmpRR => {
          let fl = CmpFlag::from(src.next().unwrap(),);
          let a = src.next().unwrap();
          let b = src.next().unwrap();
          output.push(format!("{}, {}, ${}, ${}", op, fl, a, b),);
        }
        OpCode::Not => {
          let a = src.next().unwrap();
          let b = src.next().unwrap();
          output.push(format!("{}, ${} ${}", op, a, b),);
        }
        OpCode::Call => {
          let a = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push(format!("{} {}", op, a),);
        }
        OpCode::Copy => output.push(format!(
          "{} ${} ${}",
          op,
          src.next().unwrap(),
          src.next().unwrap()
        ),),
        OpCode::MemCpy => {
          let rd = src.next().unwrap();
          let r0 = src.next().unwrap();
          output.push(format!("{} ${} ${}", op, rd, r0,),);
        }
        OpCode::SysCall => output.push(format!("{} {}", op, src.next().unwrap()),),
        OpCode::Ret => {
          let a = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push(format!("{} {}", op, a),)
        }
        OpCode::Alloc => {
          let dst = src.next().unwrap();
          let r0 = src.next().unwrap();
          output.push(format!("{} ${} ${}", op, dst, r0),);
        }
        OpCode::Dealloc => output.push(format!("{}", op),),
        OpCode::RMem => {
          let rd = src.next().unwrap();
          let r0 = src.next().unwrap();
          let i_o = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          let r_o = src.next().unwrap();
          output.push(format!("{} ${} ${} {} ${}", op, rd, r0, i_o, r_o),);
        }
        OpCode::WMem => {
          let rd = src.next().unwrap();
          let r0 = src.next().unwrap();
          let i_o = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          let r_o = src.next().unwrap();
          output.push(format!("{} ${} ${} {} ${}", op, rd, r0, i_o, r_o),);
        }
        OpCode::Push => {
          let a = src.next().unwrap();
          output.push(format!("{} ${}", op, a),);
        }
        OpCode::Pop => output.push(format!("{}", op),),
        OpCode::PopR => {
          let a = src.next().unwrap();
          output.push(format!("{} ${}", op, a),);
        }
        OpCode::Noop => output.push(format!("{}", op),),
      }
    }
    write!(f, "{:?}", output)
  }
}

impl Debug for Program {
  fn fmt(&self, f:&mut std::fmt::Formatter<'_,>,) -> std::fmt::Result {
    Display::fmt(&self, f,)
  }
}

impl Program {
  pub fn new() -> Self {
    Self { inner:Vec::new(), }
  }

  pub fn push(&mut self, value:u8,) {
    self.inner.push(value,);
  }

  pub fn extend_from_slice(&mut self, other:&[u8],) {
    self.inner.extend_from_slice(other,);
  }

  pub fn len(&self,) -> usize {
    self.inner.len()
  }

  pub fn as_slice(&self,) -> &[u8] {
    self.inner.as_slice()
  }

  pub fn as_mut_slice(&mut self,) -> &mut [u8] {
    self.inner.as_mut_slice()
  }

  pub fn save(&self, output:&str,) -> Result<(),> {
    // TODO: Add better errors?
    let mut file = File::create(output,)?;
    file.write_all(self.inner.as_slice(),)?;
    Ok((),)
  }

  pub fn load(source:&str,) -> Result<Self,> {
    // TODO: Add better errors?
    let mut file = File::open(source,)?;
    let mut inner = Vec::new();
    file.read_to_end(&mut inner,)?;
    Ok(Program { inner, },)
  }
}

#[cfg(test)]
mod test {
  use super::Program;

  #[test]
  fn ser_de() {
    let p = Program::from(&[0, 15, 20, 90,],);
    p.save("test_output.spdr",).unwrap();
    let new_p = Program::load("test_output.spdr",).unwrap();

    assert_eq!(new_p.inner, p.inner);
  }
}
