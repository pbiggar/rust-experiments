use std::sync::Arc;

use crate::runtime::*;

#[derive(Debug)]
pub enum Expr_ {
  Let {
    lhs:  String,
    rhs:  Expr,
    body: Expr,
  },
  FnCall {
    name: FunctionDesc_,
    args: im::Vector<Expr>,
  },
  Lambda {
    params: im::Vector<String>,
    body:   Expr,
  },
  Variable {
    name: String,
  },
  IntLiteral {
    val: i32,
  },
}

pub type Expr = Arc<Expr_>;
unsafe impl Send for Expr_ {}
unsafe impl Sync for Expr_ {}

use Expr_::*;

pub fn let_(lhs: &str, rhs: Expr, body: Expr) -> Expr {
  Arc::new(Let { lhs: lhs.to_string(),
                 rhs,
                 body })
}

pub fn int(val: i32) -> Expr {
  Arc::new(IntLiteral { val })
}

pub fn var(name: &str) -> Expr {
  Arc::new(Variable { name: name.to_string(), })
}

pub fn lambda(names: im::Vector<&str>, body: Expr) -> Expr {
  Arc::new(Lambda { params: names.iter().map(|n| n.to_string()).collect(),
                    body })
}

impl From<i32> for Expr_ {
  fn from(item: i32) -> Self {
    IntLiteral { val: item }
  }
}

// Stdlib function
pub fn sfn(module: &str,
           name: &str,
           version: u32,
           args: im::Vector<Expr>)
           -> Expr {
  Arc::new(FnCall { name: FunctionDesc_::FunctionDesc("dark".to_string(),
                                                      "stdlib".to_string(),
                                                      module.to_string(),
                                                      name.to_string(),
                                                      version),
                    args })
}
