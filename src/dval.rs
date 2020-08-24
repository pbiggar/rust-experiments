use crate::expr;
use im_rc as im;
use std::rc::Rc;

// use crate::{errors, expr};
use crate::errors;

#[derive(Debug)]
pub enum Dval_ {
  DInt(i32),
  DList(im::Vector<Dval>),
  DLambda(im::Vector<String>, expr::Expr),
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

pub fn int(i: i32) -> Dval {
  Rc::new(Dval_::DInt(i))
}

pub fn list(l: im::Vector<Dval>) -> Dval {
  Rc::new(Dval_::DList(l))
}
