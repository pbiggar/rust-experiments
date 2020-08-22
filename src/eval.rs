use crate::{
  dval as D, dval::Dval, errors::Error::*, expr as E, expr::Expr,
  runtime::*,
};
use im_rc as im;
use std::rc::Rc;

pub fn run(body: Expr) -> Dval {
  let environment = Environment { functions: stdlib(), };

  let st = im::HashMap::new();

  eval(&body, &st, &environment)
}
macro_rules! dfn {
  ($module:ident.$name:ident.$version:literal($ ($arg:pat),*) $body:block ) => { {
    let module = stringify!($module);
    let name = stringify!($name);
    let version = stringify!($version).to_string().parse::<u32>().unwrap();
    let fn_name = FunctionDesc_::FunctionDesc(
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
            Rc::new(
              move |args| { {
                match args.iter().map(|v| &(**v)).collect::<Vec<_>>().as_slice() {
                  [$( $arg ),*] => $body,
                  _ => {
                    Rc::new(DError((IncorrectArguments(fn_name2.clone(), args))))
                  }}}})},
                 },
                )
  }};
}
/*  */
/* #[macros::darkfn] */
/* fn int_random_0(start: int, end: int) -> List<int> { */
// *start: the first variable
// *end: the second variable
/*   D.list((*start..*end).map(int).collect()) */
/* } */

/* ( */
/*   fn_name, */
/*   StdlibFunction { */
/*     t: [] */
/*     f: */
/*       { */
/*         Rc::new( */
/*           move |args| { { */
/*             match args.iter().map(|v| &(**v)).collect::<Vec<_>>().as_slice() { */
/*               [DInt(start), DInt(end)] => $body, */
/*               _ => { */
/*                 Rc::new(DError((IncorrectArguments(fn_name2.clone(), args)))) */
/*               }}}})}, */
/*              }, */
/*             ) */
/*  */

fn stdlib() -> StdlibDef {
  use crate::dval::{Dval_::*, *};
  let fns = vec![dfn!(Int.random.0() { int(rand::random()) }),
                 dfn!(Int.range.0(DInt(start), DInt(end)) {
                   Rc::new(DList((*start..*end).map(int).collect()))
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
                      Rc::new(DList(new_list))
                 }),];
  fns.into_iter().collect()
}

fn eval(expr: &Expr, symtable: &SymTable, env: &Environment) -> Dval {
  use crate::{dval::*, expr::Expr_::*, runtime::FunctionDesc_::*};
  match &**expr {
    IntLiteral { val } => int(*val),
    Let { lhs: _,
          rhs: _,
          body, } => eval(&body, symtable, env),
    Variable { name: _ } => int(0),
    Lambda { params: _, body: _ } => int(0),
    FnCall { name:
               FunctionDesc(owner,
                            package,
                            module,
                            name,
                            version),
             args, } => {
      let fn_def = env.functions.get(&FunctionDesc(owner.clone(),
                                                   package.clone(),
                                                   module.clone(),
                                                   name.clone(),
                                                   *version));

      match fn_def {
        Option::Some(v) => {
          let args = args.into_iter()
                         .map(|arg| eval(&arg, symtable, env))
                         .collect();

          (v.f)(args)
        }
        Option::None => {
          Rc::new(Dval_::DError(MissingFunction(FunctionDesc(owner.clone(),
                                                             package.clone(),
                                                             module.clone(),
                                                             name.clone(),
                                                             *version))))
        }
      }
    }
  }
}
