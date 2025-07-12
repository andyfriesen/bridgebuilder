use std::{
    fs::File,
    io::{IoSlice, Write},
};

use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{FieldsUnnamed, Ident};

struct StringBuilder {
    chunks: Vec<String>,
}

impl StringBuilder {
    fn new() -> Self {
        StringBuilder { chunks: Vec::new() }
    }

    fn add<S : Into<String>>(&mut self, s: S) {
        self.chunks.push(s.into());
    }

    fn write(&self, file: &mut File) {
        let mut ioslices = Vec::new();
        for s in &self.chunks {
            ioslices.push(IoSlice::new(s.as_bytes()))
        }

        file.write_vectored(&ioslices).unwrap();
    }
}

struct Output {
    structs: StringBuilder,
    methods: StringBuilder,
    get_specializations: StringBuilder,
}

impl Output {
    fn new() -> Self {
        Output {
            structs: StringBuilder::new(),
            methods: StringBuilder::new(),
            get_specializations: StringBuilder::new(),
        }
    }

    fn add_method(&mut self, cxx_type_name: &str, name: &str, variant_name: &str) {
        self.methods.add(format!(
            "extern \"C\" const {}* seedoubleplus_{}To{}(const {}* self);\n",
            cxx_type_name,
            name,
            variant_name,
            name
        ));

        self.get_specializations.add(format!(
            "template <> inline const {}* get<{}::{}>(const {}* self) {{\n    return seedoubleplus_{}To{}(self);\n}}\n\n",
            cxx_type_name,
            name,
            variant_name,
            name,
            name,
            variant_name
        ));
    }

    fn write(&self) {
        let mut header = File::create("out.h").unwrap();

        self.structs.write(&mut header);
        self.methods.write(&mut header);
        self.get_specializations.write(&mut header);
    }
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

#[proc_macro_attribute]
pub fn export_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemEnum = syn::parse(item).unwrap();
    let name = &ast.ident;

    let mut tokens = TokenStream::new();
    tokens.extend(TokenStream::from(ast.to_token_stream()).into_iter());

    // let mut out = Output::new();

    // out.structs.add("// This header is machine generated!  Do not edit!\n#pragma once\n\n");

    // out.structs.add(format!("struct {} {{\n", name));

    // out.get_specializations.add(format!("\ntemplate <typename T> const T* get(const {}* self);\n\n", name));

    // for variant in &ast.variants {
    //     let variant_name = &variant.ident.to_string();

    //     match &variant.fields {
    //         syn::Fields::Unit => {
    //             out.structs.add(format!("    struct {} {{}};\n", variant_name.to_string().as_str()));
    //             tokens.extend(make_rust_matching_function_0(name, &variant.ident).into_iter());
    //         },
    //         syn::Fields::Unnamed(fu) => {
    //             let ff = first_field(fu);
    //             out.structs.add(format!("    using {} = {};\n", variant_name.to_string().as_str(), to_cpp_type(&ff)));
    //             tokens.extend(make_rust_matching_function_1(name, &variant.ident, &ff).into_iter());
    //         },
    //         _ => panic!(),
    //     };

    //     out.add_method(
    //         format!("{}::{}", name, variant_name).as_str(),
    //         name.to_string().as_str(),
    //         variant_name.to_string().as_str()
    //     );
    // }

    // out.structs.add("};\n\n");

    // out.write();

    tokens
}
