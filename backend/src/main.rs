#![feature(trace_macros)]
#![feature(box_syntax)]
#![feature(log_syntax)]

macro_rules! ivec {
  () => (
      im::Vector::new()
  );
  ($($x:expr),+ $(,)?) => (
      im::Vector::from(<[_]>::into_vec(box [$($x),+]))
  );
}

mod dval;
mod errors;
mod eval;
mod expr;
mod runtime;
use expr::*;
use im_rc as im;

fn main() -> Result<(), errors::Error> {
  let program = elet(
                     "range",
                     esfn(
    "Int",
    "range",
    0,
    ivec![eint(1), eint(100),],
  ),
                     esfn(
    "List",
    "map",
    0,
    ivec![(evar("range")),

              elambda(ivec!["i"],
               eif(
                 ebinop(
                 ebinop(
                   evar("i"),"Int",
                   "%", 0,
                   eint(15)
                  ), "Int", "==", 0, eint(0)), estr("fizzbuzz"),

                eif(
                 ebinop(
                 ebinop(
                   evar("i"),
                   "Int",
                   "%",0,
                   eint(5)
                  ), "Int", "==", 0, eint(0)), estr("buzz"),

                eif(
                 ebinop(
                 ebinop(
                   evar("i"),
                   "Int",
                   "%", 0,
                   eint(3)
                  ), "Int", "==", 0, eint(0)), estr("fizz"), esfn("Int", "toString", 0, ivec![evar("i")])   )


                )

                ))  ],
  ),
  );

  let result = eval::run(program);
  match &*result {
    dval::Dval_::DSpecial(dval::Special::Error(err)) => {
      use std::io::Write;
      let stderr = &mut ::std::io::stderr();
      let errmsg = "Error writing to stderr";

      writeln!(stderr, "error: {}", err).expect(errmsg);
      ::std::process::exit(1);
    }
    _ => {
      println!("{:?}", result);
      Ok(())
    }
  }
}
