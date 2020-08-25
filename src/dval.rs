use crate::expr;
use im_rc as im;
use std::rc::Rc;

// use crate::{errors, expr};
use crate::{errors, runtime};

#[derive(Debug)]
pub enum Dval_ {
  DInt(i32),
  DList(im::Vector<Dval>),
  DLambda(runtime::SymTable, im::Vector<String>, expr::Expr),
  DError(errors::Error),
}

pub type Dval = Rc<Dval_>;

unsafe impl Send for Dval_ {}
unsafe impl Sync for Dval_ {}

#[derive(Debug)]
pub enum DType {
  // TList(Rc<DType>),
// TLambda,
// TAny,
// NamedType(String),
}

pub fn derror(error: errors::Error) -> Dval {
  Rc::new(Dval_::DError(error))
}

pub fn dint(i: i32) -> Dval {
  Rc::new(Dval_::DInt(i))
}

pub fn dlist(l: im::Vector<Dval>) -> Dval {
  Rc::new(Dval_::DList(l))
}
