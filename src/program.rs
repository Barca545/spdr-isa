use crate::opcodes::{CmpFlag, OpCode};
use eyre::Result;
use std::{
  fmt::{Debug, Display},
  fs::File,
  io::{Read, Write},
  mem::transmute,
  ops::{Index, IndexMut, Range},
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

impl<const N: usize,> From<[u8; N],> for Program {
  fn from(value:[u8; N],) -> Self {
    Program {
      inner:Vec::from(value,),
    }
  }
}

impl From<&[u8],> for Program {
  fn from(value:&[u8],) -> Self {
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
    let mut output = String::new();
    let mut src = self.inner.clone().into_iter();

    while let Some(val,) = src.next() {
      let op = OpCode::from(val,);
      match op {
        OpCode::Load => {
          let target = src.next().unwrap();
          let num = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push_str(&format!("{} ${}, {}", op, target, num),);
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
          output.push_str(&format!("{} ${}, ${}, {}", op, target, a, b),);
        }
        OpCode::AddRR | OpCode::SubRR | OpCode::MulRR | OpCode::DivRR | OpCode::PowRR => {
          let target = src.next().unwrap();
          let a = src.next().unwrap();
          let b = src.next().unwrap();
          output.push_str(&format!("{} ${}, ${}, ${}", op, target, a, b),);
        }
        OpCode::Jmp => {
          let idx = unsafe { transmute::<[u8; 4], u32,>(src.next_chunk::<4>().unwrap(),) };
          output.push_str(&format!("{} {}", op, idx),);
        }
        OpCode::Jnz | OpCode::Jz => {
          let cond = match src.next() {
            Some(a,) if a == 2 => "EQ",
            Some(a,) => &a.to_string(),
            None => unreachable!(),
          };
          let idx = unsafe { transmute::<[u8; 4], u32,>(src.next_chunk::<4>().unwrap(),) };
          output.push_str(&format!("{} ${}, {}", op, cond, idx),);
        }
        OpCode::CmpRI => {
          let fl = CmpFlag::from(src.next().unwrap(),);
          let a = src.next().unwrap();
          let b = unsafe { transmute::<[u8; 4], f32,>(src.next_chunk::<4>().unwrap(),) };
          output.push_str(&format!("{} {}, ${}, {}", op, fl, a, b),);
        }
        OpCode::CmpRR => {
          let fl = CmpFlag::from(src.next().unwrap(),);
          let a = src.next().unwrap();
          let b = src.next().unwrap();
          output.push_str(&format!("{} {}, ${}, ${}", op, fl, a, b),);
        }
        OpCode::Not | OpCode::WriteStr => {
          let a = match src.next() {
            Some(a,) if a == 2 => "EQ",
            Some(a,) => &a.to_string(),
            None => unreachable!(),
          };
          let b = src.next().unwrap();
          output.push_str(&format!("{} ${}, ${}", op, a, b),);
        }
        OpCode::Copy | OpCode::MemCpy => {
          let rd = src.next().unwrap();
          let r0 = src.next().unwrap();
          output.push_str(&format!("{} ${}, ${}", op, rd, r0,),);
        }
        OpCode::Call | OpCode::SysCall | OpCode::Ret => {
          output.push_str(&format!("{} {}", op, src.next().unwrap()),)
        }
        OpCode::Alloc | OpCode::Realloc => {
          let dst = src.next().unwrap();
          let r0 = src.next().unwrap();
          output.push_str(&format!("{} ${}, ${}", op, dst, r0),);
        }
        OpCode::RMem | OpCode::WMem => {
          let rd = src.next().unwrap();
          let r0 = src.next().unwrap();
          let i_o = unsafe { transmute::<[u8; 4], u32,>(src.next_chunk::<4>().unwrap(),) };
          let r_o = src.next().unwrap();
          output.push_str(&format!("{} ${}, ${}, {}, ${}", op, rd, r0, i_o, r_o),);
        }
        OpCode::Dealloc | OpCode::Push | OpCode::PopR => {
          output.push_str(&format!("{} ${}", op, src.next().unwrap()),)
        }
        OpCode::Hlt | OpCode::Pop | OpCode::Noop => output.push_str(&format!("{}", op),),
      }
      output.push('\n',);
    }
    write!(f, "{output}",)
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

  /// Prepends arguments to the "front" (`Program.inner[0]`) of the
  /// [`Program`]'s inner vector.
  pub fn push_front(&mut self, args:Vec<u8,>,) {
    self.inner.splice(Range { start:0, end:0, }, args.into_iter(),);
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
  use crate::{
    opcodes::{CmpFlag, OpCode},
    registers::EQ,
  };
  use eyre::{eyre, Result};
  use std::io::Write;

  #[test]
  #[rustfmt::skip]
  fn opcode_printing_in_program_display() {
    // Test Hlt
    op_cmp([OpCode::Hlt.into(),], "Hlt",).unwrap();
    // Test Load
    op_cmp([OpCode::Load.into(), 14, 0, 0, 128, 63,], "Load $14, 1",).unwrap();
    // Test Copy
    op_cmp([OpCode::Copy.into(), 14, 15,], "Copy $14, $15",).unwrap();
    // Test MemCpy
    op_cmp([OpCode::MemCpy.into(), 14, 15,], "MemCpy $14, $15",).unwrap();
    // Test Add_RI
    op_cmp([OpCode::AddRI.into(), 14, 15, 0, 0, 128, 63,], "Add_RI $14, $15, 1",).unwrap();
    // Test Sub_RI
    op_cmp([OpCode::SubRI.into(), 14, 15, 0, 0, 128, 63,], "Sub_RI $14, $15, 1",).unwrap();
    // Test RvSubRI
    op_cmp([OpCode::RvSubRI.into(), 14, 15, 0, 0, 128, 63,], "RvSub_RI $14, $15, 1",).unwrap();
    // Test Mul_RI
    op_cmp([OpCode::MulRI.into(), 14, 15, 0, 0, 128, 63,], "Mul_RI $14, $15, 1",).unwrap();
    // Test Div_RI
    op_cmp([OpCode::DivRI.into(), 14, 15, 0, 0, 128, 63,], "Div_RI $14, $15, 1",).unwrap();
    // Test RvDivRI
    op_cmp([OpCode::RvDivRI.into(), 14, 15, 0, 0, 128, 63,], "RvDiv_RI $14, $15, 1",).unwrap();
    // Test Pow_RI
    op_cmp([OpCode::PowRI.into(), 14, 15, 0, 0, 128, 63,], "Pow_RI $14, $15, 1",).unwrap();
    // Test RvPowRI
    op_cmp([OpCode::RvPowRI.into(), 14, 15, 0, 0, 128, 63,], "RvPow_RI $14, $15, 1",).unwrap();
    // Test Add_RR
    op_cmp([OpCode::AddRR.into(), 14, 15, 16,], "Add_RR $14, $15, $16",).unwrap();
    // Test Sub_RR
    op_cmp([OpCode::SubRR.into(), 14, 15, 16,], "Sub_RR $14, $15, $16",).unwrap();
    // Test Mul_RR
    op_cmp([OpCode::MulRR.into(), 14, 15, 16,], "Mul_RR $14, $15, $16",).unwrap();
    // Test Div_RR
    op_cmp([OpCode::DivRR.into(), 14, 15, 16,], "Div_RR $14, $15, $16",).unwrap();
    // Test Pow_RR
    op_cmp([OpCode::PowRR.into(), 14, 15, 16,], "Pow_RR $14, $15, $16",).unwrap();
    // Test Cmp_RI
    op_cmp([OpCode::CmpRI.into(), CmpFlag::Eq.into(), 14, 0, 0, 128, 63,], "Cmp_RI EQ, $14, 1",).unwrap();
    // Test Cmp_RR
    op_cmp([OpCode::CmpRR.into(), CmpFlag::Gt.into(), 14, 15,], "Cmp_RR GT, $14, $15",).unwrap();
    // Test Not
    op_cmp([OpCode::Not.into(), EQ as u8, 14], "Not $EQ, $14").unwrap();
    // Test Jmp
    op_cmp([OpCode::Jmp.into(), 50, 0, 0 ,0,], "Jmp 50").unwrap();
    // Test Jz
    op_cmp([OpCode::Jz.into(), 2, 50, 0, 0 ,0,], "Jz $EQ, 50").unwrap();
    // Test Jnz
    op_cmp([OpCode::Jnz.into(), 2, 50, 0, 0 ,0,], "Jnz $EQ, 50").unwrap();
    // Test Call
    op_cmp([OpCode::Call.into(), 14,], "Call 14").unwrap();
    // Test SysCall
    op_cmp([OpCode::SysCall.into(), 14,], "SysCall 14").unwrap();
    // Test Ret
    op_cmp([OpCode::Ret.into(), 14,], "Ret 14").unwrap();
    // Test Alloc
    op_cmp([OpCode::Alloc.into(), 14, 15,], "Alloc $14, $15").unwrap();
    // Test Realloc
    op_cmp([OpCode::Realloc.into(), 14, 15,], "Realloc $14, $15").unwrap();
    // Test Dealloc
    op_cmp([OpCode::Dealloc.into(), 14,], "Dealloc $14").unwrap();
    // Test RMem
    op_cmp([OpCode::RMem.into(), 14, 15, 1, 0, 0, 0, 16,], "RMem $14, $15, 1, $16").unwrap();
    // Test WMem
    op_cmp([OpCode::WMem.into(), 14, 15, 1, 0, 0, 0, 16,], "WMem $14, $15, 1, $16").unwrap();
    // Test Push
    op_cmp([OpCode::Push.into(), 14,], "Push $14").unwrap();
    // Test Pop
    op_cmp([OpCode::Pop.into(),], "Pop").unwrap();
    // Test PopR
    op_cmp([OpCode::PopR.into(), 14,], "PopR $14").unwrap();
    // Test WriteStr
    op_cmp([OpCode::WriteStr.into(), 15, 16,], "WriteStr $15, $16").unwrap();
    // Test Noop
    op_cmp([OpCode::Noop.into(),], "Noop").unwrap();
    // Test All
    op_cmp(
    [
        OpCode::Load.into(), 14, 0, 0, 128, 63, 
        OpCode::Copy.into(), 14, 15,
        OpCode::MemCpy.into(), 14, 15,
        OpCode::AddRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::SubRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::RvSubRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::MulRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::DivRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::RvDivRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::PowRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::RvPowRI.into(), 14, 15, 0, 0, 128, 63,
        OpCode::AddRR.into(), 14, 15, 16,
        OpCode::SubRR.into(), 14, 15, 16,
        OpCode::MulRR.into(), 14, 15, 16,
        OpCode::DivRR.into(), 14, 15, 16,
        OpCode::PowRR.into(), 14, 15, 16,
        OpCode::CmpRI.into(), CmpFlag::Eq.into(), 14, 0, 0, 128, 63,
        OpCode::CmpRR.into(), CmpFlag::Gt.into(), 14, 15,
        OpCode::Not.into(), EQ as u8, 14,
        OpCode::Jmp.into(), 50, 0, 0 ,0,
        OpCode::Jz.into(), 2, 50, 0, 0 ,0,
        OpCode::Jnz.into(), 2, 50, 0, 0 ,0,
        OpCode::Call.into(), 14,
        OpCode::SysCall.into(), 14,
        OpCode::Ret.into(), 14,
        OpCode::Alloc.into(), 14, 15,
        OpCode::Realloc.into(), 14, 15,
        OpCode::Dealloc.into(), 14,
        OpCode::RMem.into(), 14, 15, 1, 0, 0, 0, 16,
        OpCode::WMem.into(), 14, 15, 1, 0, 0, 0, 16,
        OpCode::Push.into(), 14,
        OpCode::Pop.into(),
        OpCode::PopR.into(), 14,
        OpCode::WriteStr.into(), 15, 16,
        OpCode::Noop.into(),
      ], "\
      Load $14, 1\n\
      Copy $14, $15\n\
      MemCpy $14, $15\n\
      Add_RI $14, $15, 1\n\
      Sub_RI $14, $15, 1\n\
      RvSub_RI $14, $15, 1\n\
      Mul_RI $14, $15, 1\n\
      Div_RI $14, $15, 1\n\
      RvDiv_RI $14, $15, 1\n\
      Pow_RI $14, $15, 1\n\
      RvPow_RI $14, $15, 1\n\
      Add_RR $14, $15, $16\n\
      Sub_RR $14, $15, $16\n\
      Mul_RR $14, $15, $16\n\
      Div_RR $14, $15, $16\n\
      Pow_RR $14, $15, $16\n\
      Cmp_RI EQ, $14, 1\n\
      Cmp_RR GT, $14, $15\n\
      Not $EQ, $14\n\
      Jmp 50\n\
      Jz $EQ, 50\n\
      Jnz $EQ, 50\n\
      Call 14\n\
      SysCall 14\n\
      Ret 14\n\
      Alloc $14, $15\n\
      Realloc $14, $15\n\
      Dealloc $14\n\
      RMem $14, $15, 1, $16\n\
      WMem $14, $15, 1, $16\n\
      Push $14\n\
      Pop\n\
      PopR $14\n\
      WriteStr $15, $16\n\
      Noop").unwrap();
  }

  #[test]
  fn push_front_program() {
    let mut program = Program::from(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],);

    program.push_front(vec![1u8, 2, 3, 4, 5],);

    assert_eq!(program.len(), 15);
    assert_eq!(
      program.as_slice(),
      &[1u8, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
  }

  #[test]
  fn serilize_deserialize_program() {
    let p = Program::from(&[0, 15, 20, 90,],);
    p.save("test_output.spdr",).unwrap();
    let new_p = Program::load("test_output.spdr",).unwrap();

    assert_eq!(new_p.inner, p.inner);
  }

  /// Helper function for comparing the output of printing a program in tests.
  fn op_cmp<const N: usize,>(p:[u8; N], exp:&str,) -> Result<(),> {
    let mut w = Vec::new();
    let p = Program::from(p,);
    write!(&mut w, "{}", p).unwrap();
    let exp = String::from_iter([exp, "\n",],);
    let out = String::from_utf8(w,).unwrap();

    match out == exp {
      true => Ok((),),
      false => Err(eyre!(
        "assertion `left == right` failed\nleft: {out}\nright: {exp}",
      ),),
    }
  }
}
