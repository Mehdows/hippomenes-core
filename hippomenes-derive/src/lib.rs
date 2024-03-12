use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};
#[proc_macro_derive(ImmediateAccess, attributes(width))]
pub fn derive_immediate_access(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    //let width = input.attrs.get(0).expect("Field must have a defined width");
    let width = input
        .attrs
        .get(0)
        .expect("Field must have a defined width")
        .meta
        .require_list()
        .expect("path")
        .tokens
        .clone();
    let data = input.data;
    let mut match_arms: Vec<TokenStream> = vec![];
    // field_data.push(quote!(let a = 1337));
    match data {
        Data::Enum(ref data) => {
            for v in data.variants.iter() {
                let ident = v.ident.clone().into_token_stream();
                let val = v
                    .discriminant
                    .clone()
                    .expect("Field variant must have an associated value")
                    .1
                    .into_token_stream();
                let set_string = format!("csrrs zero, 0x300,");
                match_arms.push(quote!(
                    #ident => {unsafe{asm!(#set_string #val)}},
                ));
            }
        }
        _ => {
            panic!("Derive only on enum")
        }
    }

    let enum_ident = input.ident;
    let output = quote!(
        impl #enum_ident {
            #[inline]
            fn set_field(field: #enum_ident) {
                match field {
                   #(#match_arms)*
                }
            }
        }
    );
    proc_macro::TokenStream::from(output)
}
