pub struct Executable {
  identifiers,
  packages,
}

pub struct Mod {
  mods: Vec<Mod>,
  uses: Vec<Use>,
  fns: Vec<Fn>
}

pub struct Fn {
  body: Block,
}

pub enum Statement {
}

pub enum Expression {
}
