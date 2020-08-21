use crate::dval::Dval;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FunctionDesc_ {
  FunctionDesc(String, String, String, String, u32),
}

impl fmt::Display for FunctionDesc_ {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let FunctionDesc_::FunctionDesc(owner, package, module, name, version) =
      self;
    write!(f, "{}/{}/{}::{}_v{}", owner, package, module, name, version)
  }
}

pub type FuncSig = Arc<dyn Fn(im::Vector<Dval>) -> Dval>;

pub type SymTable = im::HashMap<String, Dval>;

pub struct StdlibFunction {
  pub f: FuncSig,
}

pub type StdlibDef = std::collections::HashMap<FunctionDesc_, StdlibFunction>;

pub struct Environment {
  pub functions: StdlibDef,
}
