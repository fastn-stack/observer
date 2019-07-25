extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use proc_macro::TokenStream;
use proc_macro2::Span;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use std::{env, fs::File};
use syn::Item;

#[derive(Debug, Deserialize, Clone)]
struct Event {
    critical: bool,
    result_type: String,
    fields: HashMap<String, String>,
}

lazy_static! {
    static ref EVENTS: HashMap<String, Event> = {
        let events_path =
            env::var("EVENTS_PATH").expect("Set env variable of EVENTS_PATH to use observer");
        println!("Events Path:: {}", events_path);
        let events_file =
            File::open(&events_path).expect(&format!("Not able to load {}", events_path));
        serde_json::from_reader(events_file).expect(&format!("Json parse error {}", events_path))
    };
}

#[proc_macro_attribute]
pub fn observed(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let parse_meta: syn::Meta = syn::parse(metadata).expect("Failed to parse metadata");
    let meta = parse_meta.name().to_string();
    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    let function = get_fn(item);

    let visibility = function.vis;
    let ident = function.ident;
    let inputs = function.decl.inputs;
    let output = function.decl.output;
    let block = function.block;
    let table_name = ident.to_string();
    let block = rewrite_func_block(block, &table_name);
    let is_critical = get_event(&table_name).critical;
    if meta.as_str() == "with_result" {
        (quote! {
        #visibility fn #ident(#inputs) #output {
            observe_with_result(ctx, #table_name, #is_critical, || {
                #block
            })
        }
        })
        .into()
    } else {
        (quote! {
        #visibility fn #ident(#inputs) #output {
            observe_all(ctx, #table_name, #is_critical, || {
                #block
            })
        }
        })
        .into()
    }
}

fn rewrite_func_block(mut block: Box<syn::Block>, table_name: &str) -> Box<syn::Block> {
    let mut stmts: Vec<syn::Stmt> = Vec::new();

    for st in block.stmts.into_iter() {
        match st {
            syn::Stmt::Semi(e, s) => match e {
                //                syn::Expr::Macro(m) => {
                //                    let mut new_macro = m.clone();
                //                    if m.mac.path.segments[0].ident.to_string().eq("observe_result") {
                //                        new_macro.mac.path.segments[0].ident =
                //                            syn::Ident::new("observe_result_i32", Span::call_site());
                //                    }
                //                    // stmts.push(syn::Stmt::Semi(syn::Expr::Macro(new_macro), s));
                //
                //                }
                syn::Expr::Call(c) => {
                    let call = c.clone();
                    let args = call.args.clone();
                    match *c.func {
                        syn::Expr::Path(p) => {
                            let mut path = p.clone();
                            if p.path.segments[0].ident.to_string().eq("observe_field") {
                                if let syn::Expr::Lit(l) = args[1].clone() {
                                    if let syn::Lit::Str(s) = l.lit.clone() {
                                        let func = "observe_".to_string()
                                            + &get_func(s.value(), table_name);
                                        path.path.segments[0].ident =
                                            syn::Ident::new(&func, Span::call_site());
                                    }
                                }
                            }

                            if p.path.segments[0].ident.to_string().eq("observe_result") {
                                let f_name =
                                    "observe_result_".to_string() + &get_result_type(table_name);
                                path.path.segments[0].ident =
                                    syn::Ident::new(&f_name, Span::call_site());
                            }

                            stmts.push(syn::Stmt::Semi(
                                syn::Expr::Call(syn::ExprCall {
                                    attrs: call.attrs,
                                    func: Box::new(syn::Expr::Path(syn::ExprPath {
                                        attrs: vec![],
                                        qself: None,
                                        path: path.path,
                                    })),
                                    paren_token: call.paren_token,
                                    args: call.args,
                                }),
                                s,
                            ));
                        }
                        t => stmts.push(syn::Stmt::Semi(
                            syn::Expr::Call(syn::ExprCall {
                                attrs: call.attrs,
                                func: Box::new(t),
                                paren_token: call.paren_token,
                                args: call.args,
                            }),
                            s,
                        )),
                    }
                }
                t => stmts.push(syn::Stmt::Semi(t, s)),
            },
            t => stmts.push(t),
        }
    }
    block.stmts = stmts;
    block
}

#[proc_macro_attribute]
pub fn balanced_if(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).expect("failed to parse input");

    // log_simple(&format!("{:#?}", item));
    check_item(&item);

    let output = quote! { #item };
    output.into()
}

#[proc_macro_derive(Resulty)]
pub fn derive_resulty(input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input.clone()).expect("failed to parse input");
    let struc = get_struct_name(item).replace("\"", "");
    let st = &format!("impl Resulty for {} {}", struc, "{}");
    proc_macro2::TokenStream::from_str(st).unwrap().into()
}

fn get_struct_name(item: syn::Item) -> String {
    match item {
        Item::Struct(struc) => struc.ident.to_string(),
        _ => panic!("this attribute macro can only apply on structs"),
    }
}

fn get_event(table: &str) -> Event {
    match EVENTS.get(table) {
        Some(e) => e.clone(),
        None => panic!("No table named \"{}\" in the events file", table),
    }
}

fn get_func(field: String, table: &str) -> String {
    match get_event(table).fields.get(&field) {
        Some(t) => get_rust_type(t),
        None => panic!(
            "No field named \"{}\" in the fields for the table \"{}\"",
            field, table
        ),
    }
}

fn get_result_type(table: &str) -> String {
    get_event(table).result_type.to_lowercase()
}

fn get_rust_type(storage_type: &str) -> String {
    storage_type.to_lowercase()
}

//fn get_table_name(metadata: String) -> String {
//    metadata
//}

fn get_fn(item: Item) -> syn::ItemFn {
    match item {
        Item::Fn(func) => func,
        _ => panic!("this attribute macro can only apply on functions"),
    }
}

//fn log_simple(msg: &str) {
//    use std::io::prelude::*;
//
//    let path = std::path::Path::new("/tmp/log.txt");
//    let mut file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
//    file.write_all(msg.as_bytes()).unwrap();
//    file.write_all("\n".as_bytes()).unwrap();
//}

//fn log(msg: &str, path: &str) {
//    use std::io::prelude::*;
//    let path = std::path::Path::new(path);
//    let mut file = std::fs::OpenOptions::new().write(true).open(&path).unwrap();
//    file.write_all(msg.as_bytes()).unwrap();
//    file.write_all("\n".as_bytes()).unwrap();
//}

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
