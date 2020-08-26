use crate::expr;
use im_rc as im;
use std::{fmt, rc::Rc};

// use crate::{errors, expr};
use crate::{errors, runtime};

// These are types that aren't real values, but are used to hold other information
#[derive(Debug)]
pub enum Special {
  Error(errors::Error),
}

#[derive(Debug)]
pub enum Dval_ {
  DBool(bool),
  DInt(i32),
  DStr(String),
  DList(im::Vector<Dval>),
  DLambda(runtime::SymTable, im::Vector<String>, expr::Expr),
  DSpecial(Special),
}

impl Dval_ {
  pub fn is_special(&self) -> bool {
    matches!(self, Dval_::DSpecial(_))
  }
}

pub type Dval = Rc<Dval_>;

unsafe impl Send for Dval_ {}
unsafe impl Sync for Dval_ {}

#[derive(Debug)]
pub enum DType {
  TList(Rc<DType>),
  TLambda,
  TBool,
  NamedType(String),
}

impl fmt::Display for Dval_ {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("{:?}", self))?;
    Ok(())
  }
}

impl fmt::Display for DType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("{:?}", self))?;
    Ok(())
  }
}

pub fn derror(error: errors::Error) -> Dval {
  Rc::new(Dval_::DSpecial(Special::Error(error)))
}

pub fn dbool(val: bool) -> Dval {
  Rc::new(Dval_::DBool(val))
}
pub fn dint(i: i32) -> Dval {
  Rc::new(Dval_::DInt(i))
}

pub fn dstr(val: &str) -> Dval {
  Rc::new(Dval_::DStr(val.to_string()))
}

pub fn dlist(l: im::Vector<Dval>) -> Dval {
  Rc::new(Dval_::DList(l))
}
