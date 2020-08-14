use error_chain::error_chain;
use rand;
use std::sync::Arc;

error_chain! {
  errors {
    MissingFunction(desc: FunctionDesc) {
      description("missing function")
      display("missing function")
    }
    IncorrectArguments(name: String, expected: Vec<Type>) {
      description("Incorrect Arguments")
      display("incorrect arguments calling {}", name)
    }
  }
}

type SymTable = im::HashMap<String, Dval>;

type FunctionDesc = (String, String, String, String, u32);

#[derive(Debug)]
pub enum Expr_ {
  Let(String, Expr, Expr),
  FnCall(FunctionDesc, im::Vector<Expr>),
  Lambda(im::Vector<String>, Expr),
  Variable(String),
  IntLiteral(i32),
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

type FuncSig = Arc<dyn Fn(Vec<Dval>) -> Dval>;

pub struct StdlibFunction {
  f: FuncSig,
}

type StdlibDef = std::collections::HashMap<FunctionDesc, StdlibFunction>;

struct Environment {
  functions: StdlibDef,
}

macro_rules! dfn {
  ($module:ident.$name:ident.$version:literal() { $body:expr }) => {{
    let module = stringify!($module);
    println!("module {}", module);
    let name = stringify!($name);
    println!("name {}", name);
    let version = stringify!($version).to_string().parse::<u32>().unwrap();
    println!("version {}", version);
    (
      (
        "dark".to_string(),
        "stdlib".to_string(),
        module.to_string(),
        name.to_string(),
        version,
      ),
      StdlibFunction {
        f: Arc::new(|_args| $body),
      },
    )
  }};
}
// fn!(List::map::0(list: List<a>, f:Lambda<a, b>) -> List<b> {
//   let new_list = list.members
//     .iter()
//     .map(|_dv| {
//       let environment = Environment {
//         functions: stdlib(),
//       };
//     let st = im::HashMap::new();
//     Arc::new(eval(f.body, &st, &environment))
//   })
//   .collect();
//   DList(new_list)
// }
// );

fn int(i: i32) -> Dval {
  Arc::new(DInt(i))
}

macro_rules! list {
  () => (
    Arc::new(DList(im::Vector::new()))
  );
  ($elem:expr; $n:expr) => (
    Arc::new(DList(im::Vector::new(Vec::from_elem($elem, $n))))
  );
  ($($x:expr),+ $(,)?) => (
    Arc::new(DList(im::Vector::from(
      <[_]>::into_vec(Box::new([$($x),+])))))
  );
}

// fn list(l: Vec<Dval>) -> Dval {
//   Arc::new(DList(im::Vector::from(l)))
// }

fn stdlib() -> StdlibDef {
  let fns = vec![
    dfn!(Int.random.0() { int(rand::random()) } ),
    dfn!(Int.range.0() { list!(int(1), int(2)) } ),
    // dfn!(Int::random::v0 => |_args| Dval::DInt(rand::random())),
    // dfn!(
    //   List::map::v0 => |args| match args.as_slice() {
    //       [DList(members), DLambda(_, body)] => {
    //         let new_list = members
    //           .iter()
    //           .map(|_dv| {
    //             let environment = Environment {
    //               functions: stdlib(),
    //             };
    //             let st = im::HashMap::new();
    //             Arc::new(eval(body, &st, &environment))
    //           })
    //           .collect();
    //         DList(new_list)
    //       }
    //       _ => DError(Error::from(ErrorKind::IncorrectArguments(
    //         "List.map".to_string(),
    //         vec![TList(Arc::new(NamedType("a".to_string()))), TLambda],
    //         args,
    //       ))),
    //     }
    // ),
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
  match &**expr {
    IntLiteral(val) => int(*val),
    Let(_name, _rhs, body) => eval(&body, symtable, env),
    Variable(_) => int(0),
    Lambda(_, _) => int(0),
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
            .map(|arg| eval(&arg, symtable, env))
            .collect();
          (v.f)(args)
        }
        Option::None => Arc::new(DError(Error::from(ErrorKind::MissingFunction((
          owner.clone(),
          package.clone(),
          module.clone(),
          name.clone(),
          *version,
        ))))),
      }
    }
  }
}

pub fn stdlib_fn(module: &str, name: &str, version: u32, args: im::Vector<Expr>) -> Expr {
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

pub fn run(body: Expr) -> Dval {
  let environment = Environment {
    functions: stdlib(),
  };
  let st = im::HashMap::new();
  return eval(&body, &st, &environment);
}
