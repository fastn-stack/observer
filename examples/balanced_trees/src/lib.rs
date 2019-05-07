extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Item;

fn log(msg: &str) {
    use std::io::prelude::*;

    let path = std::path::Path::new("/tmp/log.txt");
    let mut file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
    file.write_all(msg.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

#[proc_macro_attribute]
pub fn balanced_if(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    log(&format!("{:#?}", item));
    check_item(&item);

    let output = quote!{ #item };
    output.into()
}

fn check_item(item: &Item) {
    match item {
        Item::Fn(func) => for stmt in func.block.stmts.iter() {
            match stmt {
                syn::Stmt::Local(local) => match &local.init {
                    Some((_, init)) => check_expr(init),
                    None => {}
                },
                syn::Stmt::Item(i) => check_item(i),
                syn::Stmt::Expr(e) => check_expr(e),
                syn::Stmt::Semi(e, _) => check_expr(e),
            }
        },
        _ => {}
    }
}

fn check_expr(expr: &syn::Expr) {
    match expr {
        syn::Expr::Array(a) => {
            for e in a.elems.iter() {
                check_expr(e)
            }
        }
        _ => {}
    }
}
