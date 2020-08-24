use crate::{
  dval,
  dval::{Dval_::*, *},
  errors::Error::*,
  expr::Expr,
  runtime::*,
};
use im_rc as im;
use macros::stdlibfn;
use std::{iter::FromIterator, rc::Rc};

pub fn run(body: Expr) -> Dval {
  let environment = Environment { functions: stdlib(), };

  let st = im::HashMap::new();

  eval(&body, &st, &environment)
}
/* macro_rules! dfn { */
/*   ($module:ident.$name:ident.$version:literal($ ($arg:pat),*) $body:block ) => { { */
/*     let module = stringify!($module); */
/*     let name = stringify!($name); */
/*     let version = stringify!($version).to_string().parse::<u32>().unwrap(); */
/*     let fn_name = functiondesc_::functiondesc( */
/*         "dark".to_string(), */
/*         "stdlib".to_string(), */
/*         module.to_string(), */
/*         name.to_string(), */
/*         version, */
/*       ); */
/*     let fn_name2 = fn_name.clone(); */
/*     ( */
/*       fn_name, */
/*       StdlibFunction { */
/*         f: */
/*           { */
/*             Rc::new( */
/*               move |args| { { */
/*                 match args.iter().map(|v| &(**v)).collect::<Vec<_>>().as_slice() { */
/*                   [$( $arg ),*] => $body, */
/*                   _ => { */
/*                     Rc::new(DError((IncorrectArguments(fn_name2.clone(), args)))) */
/*                   }}}})}, */
/*                  }, */
/*                 ) */
/*   }}; */
/* } */
/*  */
/* #[macros::darkfn] */
/* fn int_random_0(start: int, end: int) -> List<int> { */
// *start: the first variable
// *end: the second variable
/*   D.list((*start..*end).map(int).collect()) */
/* } */

#[stdlibfn]
fn int__range__0(start: Int, end: Int) -> Dval {
  list(im::Vector::from_iter((*start..*end).map(|i| int(i))))
}

#[stdlibfn]
fn int__random__0() {
  int(rand::random())
}

#[stdlibfn]
fn list__map__0(members: List, l: Lambda) {
  {
    let new_list = members.iter()
                          .map(|_dv| {
                            let environment =
                              Environment { functions: stdlib(), };
                            let st = im::HashMap::new();
                            eval(l_body, &st, &environment)
                          })
                          .collect();
    list(new_list)
  }
}

fn stdlib() -> StdlibDef {
  #[allow(non_snake_case)]
  let fns = vec![int__random__0(), int__range__0(), list__map__0(),];
  fns.into_iter().collect()
}

fn eval(expr: &Expr, symtable: &SymTable, env: &Environment) -> Dval {
  use crate::{dval::*, expr::Expr_::*, runtime::FunctionDesc_::*};
  match &**expr {
    IntLiteral { val } => int(*val),
    Let { lhs, rhs, body } => {
      let rhs = eval(rhs, symtable, env);
      let new_symtable = symtable.update(lhs.clone(), rhs);
      eval(&body, &new_symtable, env)
    }
    Variable { name } => {
      symtable.get(name).expect("variable does not exist").clone()
    }
    /* Lambda { params: _, body: _ } => int(0), */
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
