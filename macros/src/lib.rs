use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

macro_rules! error {
    ($msg:expr, $span:expr) => {
        return TokenStream::from(syn::Error::new_spanned($span, $msg)
            .to_compile_error())
    };
}

#[proc_macro_attribute]
pub fn immutable_event(_: TokenStream, input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as syn::ItemStruct);
    let mut event_params = Vec::new();
    for field in &s.fields {
        match &field.ty {
            syn::Type::Path(p) => {
                let l = p.path.segments.last().unwrap();
                event_params.push(match &l.arguments {
                    syn::PathArguments::AngleBracketed(gen) => {
                        gen.args.first().unwrap().clone()
                    },
                    _ => error!("generics expected", l)
                });
            }
            _ => error!("type path expected", &field.ty)
        }
    }

    quote! {}.into()
}
