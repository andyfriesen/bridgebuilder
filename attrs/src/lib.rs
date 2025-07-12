
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{FieldsUnnamed, Ident};

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

    for variant in &ast.variants {
        match &variant.fields {
            syn::Fields::Unit => {
                tokens.extend(make_rust_matching_function_0(name, &variant.ident).into_iter());
            },
            syn::Fields::Unnamed(fu) => {
                let ff = first_field(fu);
                tokens.extend(make_rust_matching_function_1(name, &variant.ident, &ff).into_iter());
            },
            _ => panic!(),
        };
    }

    tokens
}
