use rand;
use std::collections::HashMap;

type SymTable = HashMap<String, Dval>;

enum Statement {
  Let(),
}

struct Body {
  statements: Vec<Statement>,
  tail: Expr,
}

type FunctionDesc = (String, String, String, String, u32);

enum Expr {
  FnCall(FunctionDesc),
  IntLiteral(i32),
}

#[derive(Debug)]
enum CompilerError {
  MissingFunction(FunctionDesc),
}

#[derive(Debug)]
enum DarkError {
  CompilerError(CompilerError),
}

#[derive(Debug)]
enum Dval {
  DInt(i32),
  DError(DarkError),
}

struct StdlibFunction {
  f: Box<dyn Fn(Vec<Dval>) -> Dval>,
}

type StdlibDef = HashMap<(String, String, String, String, u32), StdlibFunction>;

struct Environment {
  functions: StdlibDef,
}

fn stdlib() -> StdlibDef {
  let fns = vec![(
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
  )];
  return fns.into_iter().collect();
}

fn eval_statement(statement: &Statement, _symtable: &SymTable, _env: &Environment) -> () {
  match statement {
    Statement::Let() => (),
  }
}

fn eval_expr(expr: &Expr, _symtable: &SymTable, env: &Environment) -> Dval {
  match expr {
    Expr::IntLiteral(val) => Dval::DInt(*val),
    Expr::FnCall((owner, package, module, name, version)) => {
      let fn_def = env.functions.get(&(
        owner.clone(),
        package.clone(),
        module.clone(),
        name.clone(),
        version.clone(),
      ));
      match fn_def {
        Option::Some(v) => (v.f)(vec![Dval::DInt(6)]),
        Option::None => Dval::DError(DarkError::CompilerError(CompilerError::MissingFunction((
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

fn eval_body(body: &Body, symtable: &SymTable, env: &Environment) -> Dval {
  for s in &body.statements {
    eval_statement(&s, symtable, env);
  }
  eval_expr(&body.tail, symtable, env)
}

fn stdlib_fn(module: &str, name: &str, version: u32) -> Expr {
  Expr::FnCall((
    "dark".to_string(),
    "stdlib".to_string(),
    module.to_string(),
    name.to_string(),
    version,
  ))
}

fn run(body: &Body) -> Dval {
  let environment = Environment {
    functions: stdlib(),
  };
  let st = HashMap::new();
  return eval_body(body, &st, &environment);
}

fn main() {
  let program = Body {
    statements: vec![],
    tail: stdlib_fn("Int", "random", 0),
  };

  let result = run(&program);
  println!("{:?}", result);
  ()
}
