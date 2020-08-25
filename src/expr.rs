use im_rc as im;
use std::rc::Rc;

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

pub type Expr = Rc<Expr_>;
unsafe impl Send for Expr_ {}
unsafe impl Sync for Expr_ {}

use Expr_::*;

pub fn elet(lhs: &str, rhs: Expr, body: Expr) -> Expr {
  Rc::new(Let { lhs: lhs.to_string(),
                rhs,
                body })
}

pub fn eint(val: i32) -> Expr {
  Rc::new(IntLiteral { val })
}

pub fn evar(name: &str) -> Expr {
  Rc::new(Variable { name: name.to_string(), })
}

pub fn elambda(names: im::Vector<&str>, body: Expr) -> Expr {
  Rc::new(Lambda { params: names.iter()
                                .map(|n| n.to_string())
                                .collect(),
                   body })
}

impl From<i32> for Expr_ {
  fn from(item: i32) -> Self {
    IntLiteral { val: item }
  }
}
pub fn efn(owner: &str,
           package: &str,
           module: &str,
           name: &str,
           version: u32,
           args: im::Vector<Expr>)
           -> Expr {
  Rc::new(FnCall { name:
                     FunctionDesc_::FunctionDesc(owner.to_string(),
                                                 package.to_string(),
                                                 module.to_string(),
                                                 name.to_string(),
                                                 version),
                   args })
}

// Stdlib function
pub fn esfn(module: &str,
            name: &str,
            version: u32,
            args: im::Vector<Expr>)
            -> Expr {
  efn("dark", "stdlib", module, name, version, args)
}
