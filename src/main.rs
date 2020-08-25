#![feature(trace_macros)]
#![feature(box_syntax)]
#![feature(log_syntax)]
#![warn(missing_debug_implementations)]

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
  let program =
    expr::let_("range",
               sfn("Int", "range", 0, ivec![int(0), int(100),]),
               sfn("List",
                   "map",
                   0,
                   ivec![(var("range")),
                         lambda(ivec!["i"], int(0),),]));

  let result = eval::run(program);
  match &*result {
    dval::Dval_::DError(err) => {
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
