use crate::dval::Dval;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FunctionDesc_ {
  FunctionDesc(String, String, String, String, u32),
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
