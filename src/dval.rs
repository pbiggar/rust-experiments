use std::sync::Arc;

use crate::{errors, expr};

#[derive(Debug)]
pub enum Dval_ {
  DInt(i32),
  DList(im::Vector<Dval>),
  DLambda(im::Vector<String>, expr::Expr),
  DError(errors::Error),
}

pub type Dval = Arc<Dval_>;

unsafe impl Send for Dval_ {}
unsafe impl Sync for Dval_ {}

#[derive(Debug)]
pub enum Type {
  TList(Arc<Type>),
  TLambda,
  TAny,
  NamedType(String),
}

pub fn int(i: i32) -> Dval {
  Arc::new(Dval_::DInt(i))
}
