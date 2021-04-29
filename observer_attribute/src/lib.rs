#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate proc_macro;
extern crate proc_macro2;

use darling::FromMeta;
use std::collections::HashMap;
use std::str::FromStr;
use std::string::ToString;
use std::{env, fs::File};

#[derive(Debug, Deserialize, Clone)]
struct Event {
    critical: bool,
    result_type: String,
    fields: HashMap<String, String>,
}

fn get_path() -> String {
    env::var("EVENTS_PATH").unwrap_or_else(|_| {
        let mut current = std::env::current_dir().expect("current_dir not found");
        loop {
            let path = current.join("observer.json");
            if path.exists() {
                return path.to_string_lossy().to_string();
            }
            current = match current.parent() {
                Some(p) => p.to_owned(),
                None => panic!("Could not find observer.json, current={:?}", current),
            };
        }
    })
}

lazy_static! {
    static ref EVENTS: HashMap<String, Event> = {
        let events_path = get_path();

        println!("Events Path:: {}", events_path);
        let events_file =
            File::open(&events_path).unwrap_or_else(|_| panic!("Not able to load {}", events_path));
        serde_json::from_reader(events_file)
            .unwrap_or_else(|_| panic!("Json parse error {}", events_path))
    };
}

const WHITELIST_EVENTS: &[&str] = &[
    "query_by_index",
    "establish",
    "execute",
    "query_by_name",
    "execute_returning_count",
];

const WHITELIST_NAMESPACES: &[&str] = &["observer__pg", "observer__mysql"];

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    with_result: bool,
    #[darling(default)]
    without_result: bool,
    #[darling(default)]
    namespace: Option<String>,
    #[darling(default)]
    id: Option<String>,
}

#[proc_macro_attribute]
pub fn observed(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(metadata as syn::AttributeArgs);
    let args: MacroArgs = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let visibility = input_fn.vis;
    let ident = input_fn.ident;
    let inputs = input_fn.decl.inputs;
    let output = input_fn.decl.output;
    let block = input_fn.block;
    let generics = &input_fn.decl.generics;
    let where_clause = &input_fn.decl.generics.where_clause;
    let is_whitelist_event = WHITELIST_EVENTS.contains(&ident.to_string().as_str());
    let is_whitelist_namespace =
        WHITELIST_NAMESPACES.contains(&args.namespace.as_ref().unwrap_or(&"".to_string()).as_str());
    let table_name = if let Some(name_space) = args.namespace {
        name_space + "__" + &ident.to_string()
    } else {
        ident.to_string()
    };
    let (block, is_critical) = if is_whitelist_event && is_whitelist_namespace {
        (block, false)
    } else {
        (
            rewrite_func_block(block, &table_name),
            get_event(&table_name).critical,
        )
    };
    if args.with_result {
        (quote! {
        #visibility fn #ident #generics (#inputs) #output #where_clause {
            use observer::observe_fields::*;
            observer::Observe::observe_with_result(#table_name, #is_critical, || {
                #block
            })
        }
        })
        .into()
    } else {
        (quote! {
        #visibility fn #ident #generics (#inputs) #output #where_clause {
            use observer::observe_fields::*;
            observer::Observe::observe_all(#table_name, #is_critical, || {
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
                                if let syn::Expr::Lit(l) = args[0].clone() {
                                    if let syn::Lit::Str(s) = l.lit.clone() {
                                        let func = "observe_".to_string()
                                            + &get_func(s.value(), table_name);
                                        path.path.segments[0].ident =
                                            syn::Ident::new(&func, proc_macro2::Span::call_site());
                                    }
                                }
                            }

                            if p.path.segments[0].ident.to_string().eq("observe_result") {
                                let f_name =
                                    "observe_result_".to_string() + &get_result_type(table_name);
                                path.path.segments[0].ident =
                                    syn::Ident::new(&f_name, proc_macro2::Span::call_site());
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
pub fn balanced_if(
    _metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    // log_simple(&format!("{:#?}", item));
    check_item(&item);

    let output = quote! { #item };
    output.into()
}

#[proc_macro_derive(Resulty)]
pub fn derive_resulty(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input");
    let struc = get_struct_name(item).replace("\"", "");
    let st = &format!("impl Resulty for {} {}", struc, "{}");
    proc_macro2::TokenStream::from_str(st).unwrap().into()
}

fn get_struct_name(item: syn::Item) -> String {
    match item {
        syn::Item::Struct(struc) => struc.ident.to_string(),
        _ => panic!("this attribute macro can only apply on structs"),
    }
}

fn get_event(table: &str) -> Event {
    match EVENTS.get(table) {
        Some(e) => e.clone(),
        None => panic!(
            "No entry for \"{}\" in the events file: {}",
            table,
            get_path()
        ),
    }
}

fn get_func(field: String, table: &str) -> String {
    match get_event(table).fields.get(&field) {
        Some(t) => get_rust_type(t),
        None => panic!(
            "No field named \"{}\" in the fields for the table \"{}\" ({})",
            field,
            table,
            get_path()
        ),
    }
}

fn get_result_type(table: &str) -> String {
    get_event(table).result_type.to_lowercase()
}

fn get_rust_type(storage_type: &str) -> String {
    storage_type.to_lowercase()
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

fn check_item(item: &syn::Item) {
    if let syn::Item::Fn(func) = item {
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
}

fn check_expr(expr: &syn::Expr) {
    if let syn::Expr::Array(a) = expr {
        for e in a.elems.iter() {
            check_expr(e)
        }
    }
}
