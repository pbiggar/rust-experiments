#![feature(trace_macros)]
use error_chain::error_chain;
use rand;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

pub enum FunctionDesc_ {
  FunctionDesc(String, String, String, String, u32),
}

error_chain! {
  errors {
    MissingFunction(desc: FunctionDesc_) {
      description("missing function")
      display("missing function")
    }
    IncorrectArguments(name: FunctionDesc_) {
      description("Incorrect Arguments")
      display("incorrect arguments calling {:?}", name)
    }
  }
}

type SymTable = im::HashMap<String, Dval>;

#[derive(Debug)]

pub enum Expr_ {
  Let {
    var:  String,
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

impl From<i32> for Expr_ {
  fn from(item: i32) -> Self {
    IntLiteral { val: item }
  }
}

#[derive(Debug)]
pub enum Dval_ {
  DInt(i32),
  DList(im::Vector<Dval>),
  DLambda(im::Vector<String>, Expr),
  DError(Error),
}

pub type Dval = Arc<Dval_>;
pub type Expr = Arc<Expr_>;

unsafe impl Send for Dval_ {}
unsafe impl Sync for Dval_ {}
unsafe impl Send for Expr_ {}
unsafe impl Sync for Expr_ {}

#[derive(Debug)]
pub enum Type {
  TList(Arc<Type>),
  TLambda,
  TAny,
  NamedType(String),
}

use Dval_::*;
use Expr_::*;
use FunctionDesc_::FunctionDesc;

type FuncSig = Arc<dyn Fn(Vec<Dval>) -> Dval>;

pub struct StdlibFunction {
  f: FuncSig,
}

type StdlibDef = std::collections::HashMap<FunctionDesc_, StdlibFunction>;

struct Environment {
  functions: StdlibDef,
}

fn int(i: i32) -> Dval {
  Arc::new(DInt(i))
}

macro_rules! dfn {

  ($module:ident.$name:ident.$version:literal($ ($arg:pat),*) $body:block ) => { {
    let module = stringify!($module);
    let name = stringify!($name);
    let version = stringify!($version).to_string().parse::<u32>().unwrap();
    let fn_name = FunctionDesc(
        "dark".to_string(),
        "stdlib".to_string(),
        module.to_string(),
        name.to_string(),
        version,
      );
    let fn_name2 = fn_name.clone();
    (
      fn_name,
      StdlibFunction {
        f:
          {
            Arc::new(
              move |args| { {
                match args.iter().map(|v| &(**v)).collect::<Vec<_>>().as_slice() {
                  [$( $arg ),*] => $body,
                  _ => {
                    Arc::new(DError(Error::from(ErrorKind::IncorrectArguments(fn_name2.clone()))))
                  }}}})},
                 },
                )
  }};
}
//       _ => DError(Error::from(ErrorKind::IncorrectArguments(
//         "List.map".to_string(),
//         vec![TList(Arc::new(NamedType("a".to_string()))), TLambda],         args,

// trace_macros!(true);

fn stdlib() -> StdlibDef {
  let fns = vec![dfn!(Int.random.0() { int(rand::random()) }),
                 dfn!(Int.range.0(DInt(start), DInt(end)) {
                   Arc::new(DList((*start..*end).map(int).collect()))
                 }),
                 dfn!(List.map.0(DList(members), DLambda(args, body)) {
                      let new_list = members
                           .iter()
                           .map(|_dv| {
                             let environment = Environment {
                               functions: stdlib(),
                             };
                             let st = im::HashMap::new();
                             eval(body, &st, &environment)
                           })
                           .collect();
                      Arc::new(DList(new_list))
                 }),];

  return fns.into_iter().collect()
}

fn eval(expr: &Expr, symtable: &SymTable, env: &Environment) -> Dval {
  match &**expr {
    IntLiteral { val } => int(*val),
    Let { var: _,
          rhs: _,
          body, } => eval(&body, symtable, env),
    Variable { name: _ } => int(0),
    Lambda { params: _, body: _ } => int(0),
    FnCall { name: FunctionDesc(owner, package, module, name, version),
             args, } => {
      let fn_def = env.functions.get(&FunctionDesc(owner.clone(),
                                                   package.clone(),
                                                   module.clone(),
                                                   name.clone(),
                                                   version.clone()));

      match fn_def {
        Option::Some(v) => {
          let args = args.into_iter()
                         .map(|arg| eval(&arg, symtable, env))
                         .collect();

          (v.f)(args)
        }
        Option::None => {
          Arc::new(DError(Error::from(ErrorKind::MissingFunction(FunctionDesc(owner.clone(),
                                                                              package.clone(),
                                                                              module.clone(),
                                                                              name.clone(),
                                                                              *version)))))
        }
      }
    }
  }
}

pub fn stdlib_fn(module: &str,
                 name: &str,
                 version: u32,
                 args: im::Vector<Expr>)
                 -> Expr {
  Arc::new(FnCall { name: FunctionDesc("dark".to_string(),
                                       "stdlib".to_string(),
                                       module.to_string(),
                                       name.to_string(),
                                       version),
                    args })
}

pub fn run(body: Expr) -> Dval {
  let environment = Environment { functions: stdlib(), };

  let st = im::HashMap::new();

  return eval(&body, &st, &environment)
}
