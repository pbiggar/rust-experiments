use rand;
use std::rc::Rc;

type SymTable = im::HashMap<String, Dval>;

type FunctionDesc = (String, String, String, String, u32);

#[derive(Debug)]
pub enum Expr {
  Let(String, Rc<Expr>, Rc<Expr>),
  FnCall(FunctionDesc, im::Vector<Rc<Expr>>),
  Lambda(im::Vector<String>, Rc<Expr>),
  Variable(String),
  IntLiteral(i32),
}

#[derive(Debug)]
pub enum CompilerError {
  MissingFunction(FunctionDesc),
  IncorrectArguments,
}

#[derive(Debug)]
pub enum DarkError {
  CompilerError(CompilerError),
}

#[derive(Debug)]
pub enum Dval {
  DInt(i32),
  DList(im::Vector<Rc<Dval>>),
  DLambda(im::Vector<String>, Rc<Expr>),
  DError(DarkError),
}

use DarkError::*;
use Dval::*;
use Expr::*;

pub struct StdlibFunction {
  f: Box<dyn Fn(Vec<Dval>) -> Dval>,
}

type StdlibDef = std::collections::HashMap<(String, String, String, String, u32), StdlibFunction>;

struct Environment {
  functions: StdlibDef,
}

fn stdlib() -> StdlibDef {
  let fns = vec![
    (
      (
        "dark".to_string(),
        "stdlib".to_string(),
        "Int".to_string(),
        "random".to_string(),
        0,
      ),
      StdlibFunction {
        f: Box::new(|_args: Vec<Dval>| Dval::DInt(rand::random())),
      },
    ),
    (
      (
        "dark".to_string(),
        "stdlib".to_string(),
        "List".to_string(),
        "map".to_string(),
        0,
      ),
      StdlibFunction {
        f: Box::new(|args: Vec<Dval>| match args.as_slice() {
          [DList(members), DLambda(_, body)] => {
            let new_list = members
              .iter()
              .map(|dv| {
                let environment = Environment {
                  functions: stdlib(),
                };
                let st = im::HashMap::new();
                Rc::new(eval(body, &st, &environment))
              })
              .collect();
            DList(new_list)
          }
          _ => DError(CompilerError(CompilerError::IncorrectArguments)),
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
        //     f: Box::new(|args: Vec<Dval>| match args.as_slice() {
        //       [DInt(start), DInt(end)] => (start..end).map(DInt),
        //
        //       // DList((start..end)  into_iter().map(DInt).collect())
        //       _ => DError(CompilerError(CompilerError::IncorrectArguments)),
        //     }),
        //   },
        // ),
      },
    ),
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
        Option::None => DError(CompilerError(CompilerError::MissingFunction((
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

pub fn stdlib_fn(module: &str, name: &str, version: u32, args: im::Vector<Rc<Expr>>) -> Rc<Expr> {
  Rc::new(FnCall(
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
