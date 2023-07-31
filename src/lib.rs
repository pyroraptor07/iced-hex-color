#![doc = include_str!("../README.md")]

mod hex_color;

use proc_macro::TokenStream;

use crate::hex_color::hex_color_impl;

/// Creates an [iced::Color](https://docs.rs/iced/latest/iced/struct.Color.html) struct from a valid hex color code
///
/// Example usage:
/// ```rust
/// use iced::widget::text;
/// use iced_hex_color::hex_color;
///
/// # fn build_text_widget() -> iced::widget::text::Text<'static> {
/// // To create a text widget with color #78cc23a6
/// let text_style = iced::theme::Text::Color(hex_color!(#78cc23a6));
/// let text_widget = text("Hello, world!").style(text_style);
/// # text_widget
/// # }
/// ```
///
/// In the above example, the macro will generate code similar to the following:
/// ```ignore
/// iced::Color::from_rgba8(120, 204, 35, 0.65)
/// ```
///
/// The [hex_color] crate is used to parse the hex codes. See the [HexColor::parse]
/// function for more information.
///
/// [hex_color]: ::hex_color
/// [HexColor::parse]: ::hex_color::HexColor::parse()
#[proc_macro]
pub fn hex_color(input: TokenStream) -> TokenStream {
    let color_struct = quote::quote! {::iced::Color};
    hex_color_impl(input.into(), color_struct).into()
}

/// Creates an [iced_core::Color](https://docs.rs/iced_core/latest/iced_core/struct.Color.html) struct from a valid hex color code
///
/// This macro is the same as the [hex_color!] macro, except the [iced_core] crate is used
/// instead of the [iced] crate. See the [hex_color!] macro for usage details.
///
/// [iced_core]: https://docs.rs/iced_core/latest/iced_core/
/// [iced]: https://docs.rs/iced/latest/iced/
#[proc_macro]
pub fn hex_color_core(input: TokenStream) -> TokenStream {
    let color_struct = quote::quote! {::iced_core::Color};
    hex_color_impl(input.into(), color_struct).into()
}
