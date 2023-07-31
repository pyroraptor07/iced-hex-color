use ::hex_color::HexColor;
use proc_macro2::TokenStream;

pub(crate) fn hex_color_impl(hex_code: TokenStream, color_struct: TokenStream) -> TokenStream {
    // .replace() part is a hack to remove the extra space introduced
    // when converting the token stream into a string
    let hex_code_string = format!("{}", hex_code).replace("# ", "#");

    let Ok(color) = HexColor::parse(&hex_code_string) else {
        let msg = format!("'{hex_code_string}' is not a valid color hex code");
        let error = syn::parse::Error::new_spanned(hex_code, msg);
        return error.to_compile_error().into();
    };

    let (c_r, c_g, c_b, c_a_u8) = color.split();
    let c_a = c_a_u8 as f32 / 255.0;

    quote::quote! {{ #color_struct::from_rgba8(#c_r, #c_g, #c_b, #c_a) }}
}
