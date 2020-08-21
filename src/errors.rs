use crate::{dval::Dval, runtime};
use im_rc as im;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  MissingFunction(runtime::FunctionDesc_),
  IncorrectArguments(runtime::FunctionDesc_, im::Vector<Dval>),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::MissingFunction(fun) => write!(f, "Missing function: {}", fun),
      Error::IncorrectArguments(fun, actuals) => {
        write!(f, "Incorrect arguments calling {}, with {:?}", fun, actuals)
      }
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    Some(self)
  }
}
