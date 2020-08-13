use im;
use std::rc::Rc;
use xk::Expr::*;
use xk::*;

fn main() {
  let program = Rc::new(Let(
    "range".to_string(),
    stdlib_fn(
      "Int",
      "range",
      0,
      im::Vector::from(vec![Rc::new(IntLiteral(0)), Rc::new(IntLiteral(100))]),
    ),
    stdlib_fn(
      "List",
      "map",
      0,
      im::Vector::from(vec![
        Rc::new(Variable("range".to_string())),
        Rc::new(Lambda(
          im::Vector::from(vec!["i".to_string()]),
          Rc::new(IntLiteral(0)),
        )),
      ]),
    ),
  ));

  let result = run(&program);
  println!("{:?}", result);
  ()
}
