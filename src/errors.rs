use crate::{
  dval::{DType, Dval},
  runtime,
};

error_chain! {
  errors {
    MissingFunction(desc: runtime::FunctionDesc_) {
      description("missing function")
      display("missing function")
    }
    IncorrectArguments(name: runtime::FunctionDesc_, actuals: im::Vector<Dval>) {
      description("Incorrect Arguments")
      display("incorrect arguments calling {:?}", name)
    }
  }
}
