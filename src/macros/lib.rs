#![feature(box_patterns)]
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote;

use punctuated::Punctuated as Punc;
use std::iter::FromIterator;
use syn::*;

fn ident(name: &str) -> Ident {
  Ident::new(name, Span::call_site())
}

fn ident_pat(name: &str) -> Pat {
  Pat::from(PatIdent { attrs:      Vec::new(),
                       by_ref:     None,
                       mutability: None,
                       subpat:     None,
                       ident:      ident(name), })
}

fn get_arguments(ifn: ItemFn) -> Vec<(String, Type)> {
  let x =
    ifn.sig
       .inputs
       .iter()
       .map(|fn_arg| match fn_arg {
         FnArg::Receiver(_) => {
           panic!("Got \"self\", expected a simple argument name")
         }
         FnArg::Typed(PatType { pat: box Pat::Ident(pat),
                                box ty,
                                .. }) => {
           (pat.ident.to_string(), ty.clone())
         }

         _ => panic!("invalid type"),
       })
       .collect();
  println!("result: {:?}", x);
  println!("input: {:?}", ifn.sig.inputs);
  x
}

fn path(scopes: Vec<&str>, name: &str) -> Path {
  Path { leading_colon: None,
         segments:      {
           let mut r = Punc::new();
           for s in scopes {
             r.push(PathSegment { ident:     ident(s),
                                  arguments: PathArguments::None, })
           }
           r.push(PathSegment { ident:     ident(name),
                                arguments: PathArguments::None, });
           r
         }, }
}

fn variant(scopes: Vec<&str>,
           variant_name: &str,
           args: Vec<&str>)
           -> Pat {
  let elems: Punc<Pat, token::Comma> =
    Punc::from_iter(args.iter().map(|arg| ident_pat(arg)));
  Pat::TupleStruct (PatTupleStruct { attrs: Vec::new(),
                     path:  path(scopes, variant_name),
                     pat:
                       PatTuple { attrs: Vec::new(),
                                  paren_token:
                                    token::Paren { span:
                                                     Span::call_site(), },
                                  elems }, })
}

fn argument_pattern(scopes: Vec<&str>, name: &str, ty: &Type) -> Pat {
  let type_name = match ty {
    Type::Path(TypePath { path, .. }) => {
      Path::get_ident(path).unwrap()
    }
    _ => panic!("Not a path"),
  };
  match type_name.to_string().as_ref() {
    "Int" => variant(scopes, "DInt", vec![name]),
    ty => panic!("type not recognized: {}", ty),
  }
}

fn get_argument_patterns(ifn: ItemFn) -> Punc<Pat, token::Comma> {
  // For (start : int, l : lambda::<[A], B>)
  // Returns DInt(start), DLambda(l_vars, l_body)a
  Punc::from_iter(get_arguments(ifn).iter()
                                    .map(|(name, ty)| {
                                      argument_pattern(vec!["dval",
                                                            "Dval_"],
                                                       name,
                                                       ty)
                                    })
                                    .into_iter())
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
//                        }); ()
// }
//

fn get_body(ifn: ItemFn) -> Box<Block> {
  ifn.block
}
//
// fn get_types(_ifn: ItemFn)
//              -> Punc<FnArg, token::Comma> {
//   Punc::new()
// }
//
fn get_fn_name(ifn: ItemFn) -> Ident {
  let name = ifn.sig.ident.to_string();
  println!("{}", name);
  quote::format_ident!("int_range_0")
}
//
#[proc_macro_attribute]
pub fn stdlibfn(_attr: TokenStream,
                item: TokenStream)
                -> TokenStream {
  let input = syn::parse_macro_input!(item as syn::ItemFn);
  let body = get_body(input.clone());
  let argument_patterns = get_argument_patterns(input.clone());
  // let _types = get_types(input.clone());
  let _fn_name = get_fn_name(input.clone());
  //
  // take function name in form a_b_c and convert to something to insert into stdlib
  // create structure of StdlibFunction
  // add types
  // add f
  let output = quote::quote! {

    fn int_range_0() -> (FunctionDesc_, StdlibFunction) {
      let module = "Int";
      let name = "range";
      let version = 0;
    let fn_name = FunctionDesc_::FunctionDesc(
        "dark".to_string(),
        "stdlib".to_string(),
        module.to_string(),
        name.to_string(),
        version,
    );
    let fn_name2 = fn_name.clone();
      (fn_name,
     StdlibFunction {
       f:
         {
           Rc::new(
             move |args| { {
               match args.iter().map(|v| &(**v)).collect::<Vec<_>>().as_slice() {
                 [ #argument_patterns ] => #body,
                 _ => {
                   Rc::new(Dval_::DError((IncorrectArguments(fn_name2.clone(), args))))
                 }}}})},
                })}
  };
  TokenStream::from(output)
}
