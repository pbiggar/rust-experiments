use crate::runtime;

error_chain! {
  errors {
    MissingFunction(desc: runtime::FunctionDesc_) {
      description("missing function")
      display("missing function")
    }
    IncorrectArguments(name: runtime::FunctionDesc_) {
      description("Incorrect Arguments")
      display("incorrect arguments calling {:?}", name)
    }
  }
}
