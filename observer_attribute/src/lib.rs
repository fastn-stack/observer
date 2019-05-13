extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use proc_macro::TokenStream;
use syn::{Item, Expr::{Closure}, Expr, punctuated::Punctuated};
use syn::token::Or;
use proc_macro2::Span;
use std::collections::HashMap;
use serde;
use std::{env, fs::File};

enum FieldType {
    Integer,
    String
}

#[derive(Debug,Deserialize,Clone)]
struct Event {
    name: String,
    critical: bool,
    fields: HashMap<String, String>
}

lazy_static!{
    static ref count: i32 = 0;
     static ref EVENTS: HashMap<String,Event> = {
        let events_path = env::var("EVENTS_PATH").unwrap_or("".to_string());
        log(&format!("random   "),"/tmp/log4.txt");
        let events_file = File::open(events_path).expect("could not load default.json");
        let events: Vec<Event> =
                serde_json::from_reader(events_file).expect("invalid json");
        let mut map: HashMap<String,Event> = HashMap::new();
        for e in events{
            map.insert(e.name.clone(),e);
        }
        map
    };
}

//const events: HashMap<String, Event> = HashMap::new();

#[proc_macro_attribute]
pub fn observed(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let a: HashMap<String,Event> = EVENTS.clone();
    log(&format!("{:?}", a.get("policy")),"/tmp/log1.txt");

    validate(metadata.to_string());

    let table_name = get_table_name(metadata.to_string());

    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    /*
    let item = wrap(item);

    let output = quote!{ #item };
    log(&format!("{:#?}", output),"/tmp/log2.txt");
    */
    let f = get_fn(item);
    let vis = f.vis;
    let ident = f.ident;
    let inputs = f.decl.inputs;
    let output = f.decl.output;
    //let block = rewrite(f.block);
    let block = f.block;

    let output = quote!{
        #vis fn #ident(#inputs) #output {
            observe(ctx, #table_name, || {
                #block
            })
        }
    };
    log(&format!("{}", output.to_string()),"/tmp/log3.txt");
    output.into()
}

#[proc_macro_attribute]
pub fn balanced_if(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).expect("failed to parse input");

    log_simple(&format!("{:#?}", item));
    check_item(&item);

    let output = quote! { #item };
    output.into()
}

fn validate(metadata: String) {
    if false {
        panic!();
    }
}

fn get_table_name(metadata: String) -> String{
    metadata
}

fn func_to_clousre(func: &syn::ItemFn) -> syn::Expr {
    let body = func.block.clone();
    parse_quote!(|ctx| { #body })
    /*
    Expr::Closure(syn::ExprClosure{attrs: vec![],
        asyncness: None,
        movability: None,
        capture: None,
        or1_token: Or{
            spans: [Span::call_site()]
        },
        inputs: Punctuated::new(),
        or2_token: Or{
            spans: [Span::call_site()]
        },
        output: syn::ReturnType::Default,
        body: Box::new(Expr::Block(syn::ExprBlock{attrs: vec![],
            label: None,
            block: syn::Block{
                brace_token: syn::token::Brace{
                    span: Span::call_site(),
                },
                stmts: func.block.stmts.clone()
            }
        })),
    })
    */
}

fn let_expr(name: &str, value: syn::Expr) -> syn::Stmt {
    let name = quote!(name);
    parse_quote!(let #name = #value;)
    /*
    syn::Stmt::Expr(syn::Expr::Let(syn::ExprLet{
        attrs: vec![],
        let_token: syn::token::Let{span: Span::call_site()},
        eq_token: syn::token::Eq{
            spans: [Span::call_site()]
        },
        expr: Box::new(value),
        pats: Punctuated::new()
    }))
    */
}

fn call_func(name: &str) -> syn::Stmt {
    let name = quote!(name);
    parse_quote!(return #name(ctx);)
    /*
    syn::Stmt::Expr(syn::Expr::Call(syn::ExprCall{
        attrs: vec![],
        func: Box::new(name),
        paren_token: syn::token::Paren{span: Span::call_site()},
        args: Punctuated::new()
    }))
    */
}

fn get_fn(item: Item) -> syn::ItemFn {
    match item {
        Item::Fn(func) =>{
            func
        },
        _ => panic!("this attribute macro can only apply on functions")
    }
}

fn wrap(item: Item) -> Item {
    match item {
        Item::Fn(mut func) =>{
            let c = func_to_clousre(&func);
            func.block.stmts = vec![let_expr("c", c), call_func("c")];
            syn::Item::Fn(func)
        },
        _ => panic!("this attribute macro can only apply on functions")
    }
}

fn log_simple(msg: &str) {
    use std::io::prelude::*;

    let path = std::path::Path::new("/tmp/log.txt");
    let mut file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
    file.write_all(msg.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

fn log(msg: &str,path: &str) {
    use std::io::prelude::*;
    let path = std::path::Path::new(path);
    let mut file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
    file.write_all(msg.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

fn check_item(item: &Item) {
    match item {
        Item::Fn(func) => {
            for stmt in func.block.stmts.iter() {
                match stmt {
                    syn::Stmt::Local(local) => match &local.init {
                        Some((_, init)) => check_expr(init),
                        None => {}
                    },
                    syn::Stmt::Item(i) => check_item(i),
                    syn::Stmt::Expr(e) => check_expr(e),
                    syn::Stmt::Semi(e, _) => check_expr(e),
                }
            }
        }
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