use std::{fs, io};

use oma::executable::Executable;

use crate::{
  parse::{ParseError, Parser},
  span::Spanned,
};

pub enum CompileError {
  Io(io::Error),
  Parse(Vec<Spanned<ParseError>>),
}

pub struct Compiler {
  executable: Executable,
}

impl Compiler {
  pub fn new() -> Compiler {
    Compiler {
      executable: Executable::new(),
    }
  }

  pub fn compile(mut self, path: &str) -> Result<Executable, CompileError> {
    self.file(path)?;
    Ok(self.executable)
  }

  pub fn file(&mut self, path: &str) -> Result<(), CompileError> {
    let source = fs::read_to_string(path).map_err(CompileError::Io)?;
    let file = Parser::new(&source).parse().map_err(CompileError::Parse)?;

    Ok(())
  }
}
