use crate::dval::Dval;
use futures::future::BoxFuture;
use std::{fmt, future::Future, pin::Pin, sync::Arc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FunctionDesc_ {
  FunctionDesc(String, String, String, String, u32),
}

impl fmt::Display for FunctionDesc_ {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let FunctionDesc_::FunctionDesc(owner,
                                    package,
                                    module,
                                    name,
                                    version) = self;
    write!(f,
           "{}/{}/{}::{}_v{}",
           owner, package, module, name, version)
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Caller {
  Toplevel(TLID),
  Code(TLID, ID),
}

impl Caller {
  pub fn to_tlid(&self) -> TLID {
    match self {
      Caller::Toplevel(tlid) => *tlid,
      Caller::Code(tlid, _) => *tlid,
    }
  }
}

// Two lifetimes: the execstate has to live as long as the boxed fn
pub type FuncSig<'a, 'b> = Box<dyn Fn(&'b crate::eval::ExecState,
                                    Vec<Dval>)
                                    -> BoxFuture<'a, Dval>
                                 + Send
                                 + Sync>;

pub type SymTable = im::HashMap<String, Dval>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TLID {
  TLID(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum ID {
  ID(u64),
}

pub fn gid() -> ID {
  ID::ID(rand::random())
}

pub fn gtlid() -> TLID {
  TLID::TLID(rand::random())
}

pub struct StdlibFunction<'a, 'b> {
  pub f: FuncSig<'a, 'b>,
}

impl fmt::Debug for StdlibFunction<'_, '_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("function")
  }
}

pub type StdlibDef<'a, 'b> =
  std::collections::HashMap<FunctionDesc_, StdlibFunction<'a, 'b>>;

pub struct Environment<'a, 'b> {
  pub functions: StdlibDef<'a, 'b>,
}

unsafe impl Send for Environment<'_, '_> {}
