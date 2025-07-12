#![feature(formatting_options)]

mod builder;

use std::{fmt::{Display, Formatter, FormattingOptions}, fs, io::{Error, Write}};

use builder::Builder;
use proc_macro2::{Span, TokenStream};
use syn::{Attribute, FieldsUnnamed, File, Ident, ItemEnum};
use quote::{format_ident, quote, ToTokens};

pub fn build(sources: &[&str]) -> Result<(), Error> {
    let mut builder = Builder::new();

    for src_path in sources {
        let bytes = fs::read(src_path).unwrap();
        let source = String::from_utf8(bytes).unwrap();

        let syntax: File = syn::parse_str(&source).unwrap();

        build_one(&mut builder, syntax)?;
    }

    builder.write("output.h")?;

    // let mut bindings_rs = fs::File::create("bindings.rs")?;
    let mut bindings_rust = String::new();
    let mut formatter = Formatter::new(&mut bindings_rust, FormattingOptions::new());

    builder.rust_module.fmt(&mut formatter).unwrap();

    fs::File::create("bindings.rs")?.write(bindings_rust.as_bytes())?;

    Ok(())
}

fn to_cpp_type(rust_type: &Ident) -> Ident {
    if rust_type == "i32" {
        Ident::new("int", Span::call_site().into())
    } else {
        rust_type.clone()
    }
}

fn first_field(fu: &FieldsUnnamed) -> Ident {
    for field in &fu.unnamed {
        match &field.ty {
            syn::Type::Path(tp) => {
                assert!(tp.qself.is_none(), "Can't handle qself types!");

                assert!(tp.path.segments.len() == 1, "Can't handle multi-arg variants yet");
                for p in &tp.path.segments {
                    assert!(p.arguments.is_none(), "Can't handle path arguments!");
                    return p.ident.clone()
                }

                return Ident::new("void", Span::call_site().into())
            },

            _ =>
                panic!("I only know how to handle boring types a::b::c")
        }
    }

    panic!("No field?");
}

fn make_rust_matching_function_0(name: &Ident, variant_name: &Ident) -> TokenStream {
    let fn_name = format_ident!("seedoubleplus_{}To{}", name, variant_name);

    quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn #fn_name(this: &#name) -> Option<&#name> {
            match this {
                #name::#variant_name => Some(&this),
                _ => None
            }
        }
    }.to_token_stream().into()
}

fn make_rust_matching_function_1(name: &Ident, variant_name: &Ident, variant_type: &Ident) -> TokenStream {
    let fn_name = format_ident!("seedoubleplus_{}To{}", name, variant_name);

    quote! {
        #[unsafe(no_mangle)]
        pub extern "C" fn #fn_name(this: &#name) -> Option<&#variant_type> {
            match this {
                #name::#variant_name(a) => Some(&a),
                _ => None
            }
        }
    }.to_token_stream().into()
}

fn export_enum(builder: &mut Builder, item: &ItemEnum) {
    let name = &item.ident;

    builder.structs.add(format!("struct {} {{\n", name));

    builder.get_specializations.add(format!("\ntemplate <typename T> const T* get(const {}* self);\n\n", name));

    for variant in &item.variants {
        let variant_name = &variant.ident.to_string();

        match &variant.fields {
            syn::Fields::Unit => {
                builder.structs.add(format!("    struct {} {{}};\n", variant_name.to_string().as_str()));
                builder.rust_module.extend(make_rust_matching_function_0(name, &variant.ident).into_iter());
            },
            syn::Fields::Unnamed(fu) => {
                let ff = first_field(fu);
                builder.structs.add(format!("    using {} = {};\n", variant_name.to_string().as_str(), to_cpp_type(&ff)));
                builder.rust_module.extend(make_rust_matching_function_1(name, &variant.ident, &ff).into_iter());
            },
            _ => panic!(),
        };

        builder.add_method(
            format!("{}::{}", name, variant_name).as_str(),
            name.to_string().as_str(),
            variant_name.to_string().as_str()
        );
    }

    builder.structs.add("};\n\n");
}

fn build_one(builder: &mut Builder, syntax: File) -> Result<(), Error> {
    let _ = &builder;

    for item in &syntax.items {
        if let syn::Item::Enum(item_enum) = item {
            if !has_attr(&item_enum.attrs) {
                continue;
            }

            export_enum(builder, item_enum);
        }
    }

    Ok(())
}

fn has_attr(attrs: &Vec<Attribute>) -> bool {
    for attr in attrs {
        if attr.path().is_ident("export_enum") {
            return true;
        }
    }
    return false;
}
