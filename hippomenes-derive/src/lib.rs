use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};
#[proc_macro_derive(CSRAccess, attributes(width, offset, address))]
pub fn derive_csr_access(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let mut width_t: Option<TokenStream> = None;
    let mut offset_t: Option<TokenStream> = None;
    let mut address_t: Option<TokenStream> = None;
    for attr in input.attrs {
        let meta = attr.meta.require_name_value();
        match meta {
            Ok(meta) => {
                if meta.path.is_ident("width") {
                    width_t = Some(meta.value.to_token_stream())
                } else if meta.path.is_ident("offset") {
                    offset_t = Some(meta.value.to_token_stream())
                } else if meta.path.is_ident("address") {
                    address_t = Some(meta.value.to_token_stream())
                }
            }
            Err(_) => {}
        }
    }
    let width_t = width_t.expect("Field width not specified, expected #[width = <WIDTH>]");
    let offset_t = offset_t.expect("Field offset not specified, expected #[offset = <OFFSET>]");
    let address_t =
        address_t.expect("Field address not specified, expected #[address = <ADDRESS>]");
    let width: u8 = width_t
        .to_string()
        .parse()
        .expect("Expected specified width to be a u8");
    let offset: u8 = offset_t
        .to_string()
        .parse()
        .expect("Expected specified offset to be a u8");

    if width == 0 {
        panic!("Invalid field width, expected 0 < width");
    }
    if offset + width > 32 {
        panic!(
            "Invalid offset and width combination, on 32-bit architectures offset + width <= 32 is expected"
        )
    }

    let enum_ident = input.ident;

    let output = quote!(
        // THIS WILL BREAK IF THRESHOLD IS SET MORE THAN ONCE
        impl #enum_ident {
            set_field!(#address_t, #width, #offset_t);
            write_field!(#address_t, #width, #offset_t);
            read_field_as_usize!(#address_t, #width, #offset_t);
            clear_field!(#address_t, #width, #offset_t);
        }
    );

    // TODO: Actual CSRRW interface instead of CSRRWI for incompatible fields
    proc_macro::TokenStream::from(output)
}
