use error_chain::error_chain;
use rand;
use std::sync::Arc;

error_chain! {
  errors {
    MissingFunction(desc: FunctionDesc) {
      description("missing function")
      display("missing function")
    }
    IncorrectArguments(name: String, expected: Vec<Type>, got: Vec<Dval>) {
      description("Incorrect Arguments")
      display("incorrect arguments calling {}", name)
    }
  }
}

type SymTable = im::HashMap<String, Dval>;

type FunctionDesc = (String, String, String, String, u32);

#[derive(Debug)]
pub enum Expr {
  Let(String, Arc<Expr>, Arc<Expr>),
  FnCall(FunctionDesc, im::Vector<Arc<Expr>>),
  Lambda(im::Vector<String>, Arc<Expr>),
  Variable(String),
  IntLiteral(i32),
}

#[derive(Debug)]
pub enum Dval {
  DInt(i32),
  DList(im::Vector<Arc<Dval>>),
  DLambda(im::Vector<String>, Arc<Expr>),
  DError(Error),
}

unsafe impl Send for Dval {}

#[derive(Debug)]
pub enum Type {
  TList(Arc<Type>),
  TLambda,
  TAny,
  NamedType(String),
}

use Dval::*;
use Expr::*;
use Type::*;

type FuncSig = Arc<dyn Fn(Vec<Dval>) -> Dval>;

pub struct StdlibFunction {
  f: FuncSig,
}

type StdlibDef = std::collections::HashMap<FunctionDesc, StdlibFunction>;

struct Environment {
  functions: StdlibDef,
}

macro_rules! dfn {
  ($module:expr, $name:expr, $body:expr) => {
    (
      (
        "dark".to_string(),
        "stdlib".to_string(),
        $module.to_string(),
        $name.to_string(),
        0,
      ),
      StdlibFunction { f: Arc::new($body) },
    )
  };
}

fn stdlib() -> StdlibDef {
  let fns = vec![
    dfn!("Int", "random", |_args| Dval::DInt(rand::random())),
    dfn!("List", "map", |args: Vec<Dval>| match args.as_slice() {
      [DList(members), DLambda(_, body)] => {
        let new_list = members
          .iter()
          .map(|_dv| {
            let environment = Environment {
              functions: stdlib(),
            };
            let st = im::HashMap::new();
            Arc::new(eval(body, &st, &environment))
          })
          .collect();
        DList(new_list)
      }
      _ => DError(Error::from(ErrorKind::IncorrectArguments(
        "List.map".to_string(),
        vec![TList(Arc::new(NamedType("a".to_string()))), TLambda],
        args,
      ))),
    }),
    // (
    //   (
    //     "dark".to_string(),
    //     "stdlib".to_string(),
    //     "List".to_string(),
    //     "range".to_string(),
    //     0,
    //   ),
    //   StdlibFunction {
    //     f: Arc::new(|args: Vec<Dval>| match args.as_slice() {
    //       [DInt(start), DInt(end)] => (start..end).map(DInt),
    //
    //       // DList((start..end)  into_iter().map(DInt).collect())
    //       _ => DError(CompilerError(CompilerError::IncorrectArguments)),
    //     }),
    //   },
    // ),
  ];
  return fns.into_iter().collect();
}

fn eval(expr: &Expr, symtable: &SymTable, env: &Environment) -> Dval {
  match expr {
    IntLiteral(val) => DInt(*val),
    Let(_name, _rhs, body) => eval(body, symtable, env),
    Variable(_) => DInt(0),
    Lambda(_, _) => DInt(0),
    FnCall((owner, package, module, name, version), args) => {
      let fn_def = env.functions.get(&(
        owner.clone(),
        package.clone(),
        module.clone(),
        name.clone(),
        version.clone(),
      ));
      match fn_def {
        Option::Some(v) => {
          let args = args
            .into_iter()
            .map(|arg| eval(arg, symtable, env))
            .collect();
          (v.f)(args)
        }
        Option::None => DError(Error::from(ErrorKind::MissingFunction((
          owner.clone(),
          package.clone(),
          module.clone(),
          name.clone(),
          *version,
        )))),
      }
    }
  }
}

pub fn stdlib_fn(module: &str, name: &str, version: u32, args: im::Vector<Arc<Expr>>) -> Arc<Expr> {
  Arc::new(FnCall(
    (
      "dark".to_string(),
      "stdlib".to_string(),
      module.to_string(),
      name.to_string(),
      version,
    ),
    args,
  ))
}

pub fn run(body: &Expr) -> Dval {
  let environment = Environment {
    functions: stdlib(),
  };
  let st = im::HashMap::new();
  return eval(body, &st, &environment);
}
