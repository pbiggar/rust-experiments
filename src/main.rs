// Simple and robust error handling with error-chain!
// Use this as a template for new projects.

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
  // Create the Error, ErrorKind, ResultExt, and Result types
  error_chain! {}
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).

fn main() {
  if let Err(ref e) = run() {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "error: {}", e).expect(errmsg);

    for e in e.iter().skip(1) {
      writeln!(stderr, "caused by: {}", e).expect(errmsg);
    }

    if let Some(backtrace) = e.backtrace() {
      writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }

    ::std::process::exit(1);
  }
}

// The above main gives you maximum control over how the error is
// formatted. If you don't care (i.e. you want to display the full
// error during an assert) you can just call the `display_chain` method
// on the error object
#[allow(dead_code)]
fn alternative_main() {
  if let Err(ref e) = run() {
    use error_chain::ChainedError;
    use std::io::Write; // trait which holds `display_chain`
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
    ::std::process::exit(1);
  }
}

// macro_rules! equations_impl {
//   (() -> ($module:ident.$name:ident.$version:literal( $($params:tt)* ) ) ) => {
//     Arc::new(Expr_::FnCall { name : ("dark", "stdlib", stringify!($module), stringify!($name), $version) })
//   };
//   (([> pattern a <] $($rest:tt)*) -> ($($output:tt)*)) => {
//       equations_impl!(($($rest)*) -> ($(output)*) [> new output <] );
//   };
//   // (([> pattern b <] $($rest:tt)*) -> ([> current output <])) => {
//   //     equations_impl!(($($rest)*) -> ($(output)*) [> new output <] );
//   // };
// }
// macro_rules! equations {
//   ($($input:tt)*) => {
//       {
//           equations_impl!(($($input)*) -> ())
//       }
//   };
// }

macro_rules! prog {
  ($module:ident.$name:ident.$version:literal( $($param:tt),* ) ) => {
    Arc::new(Expr_::FnCall { name: xk::FunctionDesc_::FunctionDesc("dark".to_string(),
                                    "stdlib".to_string(),
                                    stringify!($module).to_string(),
                                    stringify!($name).to_string(),
                                    $version),

                            args: im::Vector::from(vec!( $( prog!($param), )* ))})
  };

  ($literal:literal) => {
    Arc::new(Expr_::from($literal))
  }
}

// Use this macro to auto-generate the main above. You may want to
// set the `RUST_BACKTRACE` env variable to see a backtrace.
// quick_main!(run);

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> xk::Result<()> {
  use std::sync::Arc;
  use xk::{Expr_, Expr_::*};
  let program = Arc::new(Expr_::from(5));

  let program = prog! { Int.range.0(0, 100) };
  // let program = Arc::new(Let(
  //     "range".to_string(),
  //     stdlib_fn(
  //         "Int",
  //         "range",
  //         0,
  //         im::Vector::from(vec![Arc::new(IntLiteral(0)), Arc::new(IntLiteral(100))]),
  //     ),
  //     stdlib_fn(
  //         "List",
  //         "map",
  //         0,
  //         im::Vector::from(vec![
  //             Arc::new(Variable("range".to_string())),
  //             Arc::new(Lambda(
  //                 im::Vector::from(vec!["i".to_string()]),
  //                 Arc::new(IntLiteral(0)),
  //             )),
  //         ]),
  //     ),
  // ));

  let result = xk::run(program);
  // match &*result {
  //     Dval_::DError(err) => Err(move *err),
  //     _ => {
  //         println!("{:?}", result);
  //         Ok(())
  //     }
  // }
  println!("{:?}", result);
  Ok(())
}
