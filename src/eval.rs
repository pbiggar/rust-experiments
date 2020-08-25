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

  eval(body, st, &environment)
}
/* #[macros::darkfn] */
/* fn int_random_0(start: int, end: int) -> List<int> { */
// *start: the first variable
// *end: the second variable
/*   D.list((*start..*end).map(int).collect()) */
/* } */

#[stdlibfn]
fn int__range__0(start: Int, end: Int) -> Dval {
  dlist(im::Vector::from_iter((*start..*end).map(|i| dint(i))))
}

#[stdlibfn]
fn int__random__0() {
  dint(rand::random())
}

#[stdlibfn]
fn list__map__0(members: List, l: Lambda) {
  {
    let new_list =
      members.iter()
             .map(|_dv| {
               let environment = Environment { functions: stdlib(), };
               let st = l_symtable;
               eval(l_body.clone(), st.clone(), &environment)
             })
             .collect();
    dlist(new_list)
  }
}

fn stdlib() -> StdlibDef {
  #[allow(non_snake_case)]
  let fns = vec![int__random__0(), int__range__0(), list__map__0(),];
  fns.into_iter().collect()
}

fn eval(expr: Expr, symtable: SymTable, env: &Environment) -> Dval {
  use crate::{dval::*, expr::Expr_::*};
  match &*(expr) {
    IntLiteral { val } => dint(*val),
    Let { lhs, rhs, body } => {
      let rhs = eval(rhs.clone(), symtable.clone(), env);
      let new_symtable = symtable.update(lhs.clone(), rhs);
      eval(body.clone(), new_symtable, env)
    }
    Variable { name } => {
      symtable.get(name).expect("variable does not exist").clone()
    }
    Lambda { params, body } => {
      Rc::new(DLambda(symtable, params.clone(), body.clone()))
    }
    FnCall { name, args } => {
      let fn_def = env.functions.get(name);

      match fn_def {
        Option::Some(v) => {
          let args =
            args.into_iter()
                .map(|arg| eval(arg.clone(), symtable.clone(), env))
                .collect();

          (v.f)(args)
        }
        Option::None => derror(MissingFunction(name.clone())),
      }
    }
  }
}
