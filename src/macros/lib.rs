#![feature(box_patterns)]
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

use syn::*;

// fn ident(name: &str) -> Pat {
//   Pat::from(PatIdent { attrs:      Vec::new(),
//                        by_ref:     None,
//                        mutability: None,
//                        subpat:     None,
//                        ident:      Ident::new(name,
//                                               Span::call_site()), })
// }
//
fn punctuated<A: Copy>(items: Vec<A>)
                       -> punctuated::Punctuated<A, token::Comma> {
  let mut segments = punctuated::Punctuated::new();
  items.iter().map(|item| segments.push(*item));
  segments
}

// fn variant(name: &str, args: Vec<&str>) -> Pat {
//   let _segments = punctuated(vec![name.to_string()]);
//   let elems =
//     punctuated(args.iter().map(|name| ident(name)).collect());
//   PatTuple { attrs: Vec::new(),
//              paren_token: token::Paren { span: Span::call_site(), },
//              elems }.into()
// }
//
// fn names(arg: &str, ty: &str) -> String {
//   match ty {
//     "int" =>   Pat::TupleStruct(PatTupleStruct {attrs: Vec::new(), path: { leading_colon : false, segments : "DInt" }}),
//     _ => panic!("unknown name"),
//   }
// }

fn get_argument_patterns(
  _ifn: ItemFn)
  -> punctuated::Punctuated<Pat, token::Comma> {
  // punctuated(vec![])
  punctuated::Punctuated::new()
}

fn get_body(ifn: ItemFn) -> Box<Block> {
  ifn.block
}

fn get_types(ifn: ItemFn)
             -> punctuated::Punctuated<FnArg, token::Comma> {
  punctuated::Punctuated::new()
}

fn get_fn_name(ifn: ItemFn) -> String {
  "".to_string()
}

// turn (start: int) into (DInt(start));
// turn (l: Lambda) into (DLambda(l_names, l_body));
// fn process_sig(mut sig: syn::Signature) -> () {
//   sig.inputs.iter_mut().map(|arg| match &arg {
//                          _ => (),
//                          Typed(syn::PatType { pat:
//                                                 box ref x,
//                                               ty:
//                                                 box ref t,
//                                               .. }) => (),
//                        });
//   ()
// }
//
#[proc_macro_attribute]
pub fn darkfn(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(item as syn::ItemFn);
  let body = get_body(input.clone());
  let _argument_patterns = get_argument_patterns(input.clone());
  let _types = get_types(input.clone());
  let fn_name = get_fn_name(input.clone());

  // take function name in form a_b_c and convert to something to insert into stdlib
  // create structure of StdlibFunction
  // add types
  // add f
  let output = quote! {
    (
      #fn_name,
      StdlibFunction {
        t: vec![  ]
        f:
          {
            Rc::new(
              move |args| { {
                match args.iter().map(|v| &(**v)).collect::<Vec<_>>().as_slice() {
                  [ ] => #body,
                  _ => {
                    Rc::new(DError((IncorrectArguments(fn_name2.clone(), args))))
                  }}}})},
                 },
                )
  };
  TokenStream::from(output)
}
