fn parse_metadata(metadata: TokenStream) {
    let attr_args = parse_macro_input!(metadata as AttributeArgs);
    //let parse_meta: syn::Meta = syn::parse(metadata).expect("Failed to parse metadata");
    println!("{:?}", attr_args.len());
    for attr in attr_args {
        match attr {
            NestedMeta::Meta(meta) => match meta {
                Meta::Word(ident) => println!("Word {:?}", ident.to_string()),
                Meta::List(metalist) => println!("Meta List {:?}", metalist.ident),
                Meta::NameValue(name_value) => {
                    println!("NameValue {:?}", name_value.ident);
                    match name_value.lit {
                        Lit::Str(lit_str) => println!("LitStr {:?}", lit_str.value()),
                        _ => {}
                    }
                }
            },
            NestedMeta::Literal(lit) => println!("Literal {:#?}", format!("{:?}", lit)),
        }
    }
}
