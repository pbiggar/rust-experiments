use crate::{
  dval,
  dval::{Dval_::*, *},
  errors::Error::*,
  expr::Expr,
  runtime::*,
};
use im_rc as im;
use itertools::Itertools;
use macros::stdlibfn;
use std::{iter::FromIterator, rc::Rc};

pub struct ExecState {
  caller: Caller,
}

pub fn run(state: &ExecState, body: Expr) -> Dval {
  let environment = Environment { functions: stdlib(), };

  let st = im::HashMap::new();

  eval(state, body, st, &environment)
}
/* #[macros::darkfn] */
/* fn int_random_0(start: int, end: int) -> List<int> { */
// *start: the first variable
// *end: the second variable
/*   D.list((*start..*end).map(int).collect()) */
/* } */

fn stdlib() -> StdlibDef {
  #[stdlibfn]
  fn int__toString__0(a: Int) {
    dstr(&*format!("{}", *a))
  }

  #[stdlibfn]
  fn int__range__0(start: Int, end: Int) -> Dval {
    dlist(im::Vector::from_iter((*start..*end).map(|i| dint(i))))
  }

  #[stdlibfn]
  fn int__random__0() {
    dint(rand::random())
  }

  #[stdlibfn]
  fn int__eq__0(a: Int, b: Int) {
    dbool(*a == *b)
  }

  #[stdlibfn]
  fn int__mod__0(a: Int, b: Int) {
    dint(*a % *b)
  }

  #[stdlibfn]
  fn list__map__0(members: List, l: Lambda) {
    {
      let new_list =
        members.iter()
               .map(|dv| {
                 let environment =
                   Environment { functions: stdlib(), };
                 let st =
                   l_symtable.update(l_vars[0].clone(), dv.clone());
                 let result = eval(state,
                                   l_body.clone(),
                                   st.clone(),
                                   &environment);
                 if result.is_special() {
                   return Err(result)
                 }
                 Ok(result)
               })
               .fold_results(im::Vector::new(), |mut accum, item| {
                 accum.push_back(item);
                 accum
               });
      match new_list {
        Ok(r) => dlist(r),
        Err(special) => special,
      }
    }
  }

  let fns = vec![int__random__0(),
                 int__range__0(),
                 list__map__0(),
                 int__toString__0(),
                 int__eq__0(),
                 int__mod__0()];

  fns.into_iter().collect()
}

fn eval(state: &ExecState,
        expr: Expr,
        symtable: SymTable,
        env: &Environment)
        -> Dval {
  use crate::{dval::*, expr::Expr_::*};
  match &*expr {
    IntLiteral { id: _, val } => dint(*val),
    StringLiteral { id: _, val } => dstr(val),
    Blank { id } => {
      dincomplete(&Caller::Code(state.caller.to_tlid(), *id))
    }
    Let { id: _,
          lhs,
          rhs,
          body, } => {
      let rhs = eval(state, rhs.clone(), symtable.clone(), env);
      let new_symtable = symtable.update(lhs.clone(), rhs);
      eval(state, body.clone(), new_symtable, env)
    }
    Variable { id: _, name } => {
      symtable.get(name).expect("variable does not exist").clone()
    }
    Lambda { id: _,
             params,
             body, } => {
      Rc::new(DLambda(symtable, params.clone(), body.clone()))
    }
    If { id,
         cond,
         then_body,
         else_body, } => {
      let result = eval(state, cond.clone(), symtable.clone(), env);
      match *result {
        DBool(true) => {
          eval(state, then_body.clone(), symtable.clone(), env)
        }
        DBool(false) => eval(state, else_body.clone(), symtable, env),
        _ => dcode_error(&state.caller,
                         *id,
                         InvalidType(result, dval::DType::TBool)),
      }
    }
    BinOp { id, lhs, op, rhs } => {
      let fn_def = env.functions.get(op);

      match fn_def {
        Option::Some(v) => {
          let lhs =
            eval(state, lhs.clone(), symtable.clone(), env.clone());
          let rhs =
            eval(state, rhs.clone(), symtable.clone(), env.clone());
          let state = ExecState { caller:
                                    Caller::Code(state.caller
                                                      .to_tlid(),
                                                 *id),
                                  ..*state };

          (v.f)(&state, vec![lhs, rhs])
        }
        Option::None => {
          derror(&state.caller, MissingFunction(op.clone()))
        }
      }
    }

    FnCall { id, name, args } => {
      let fn_def = env.functions.get(name);

      match fn_def {
        Option::Some(v) => {
          let args: Vec<Dval> =
            args.into_iter()
                .map(|arg| {
                  eval(&state, arg.clone(), symtable.clone(), env)
                })
                .collect();
          let state = ExecState { caller:
                                    Caller::Code(state.caller
                                                      .to_tlid(),
                                                 *id),
                                  ..*state };

          (v.f)(&state, args)
        }
        Option::None => {
          derror(&state.caller, MissingFunction(name.clone()))
        }
      }
    }
  }
}
