use iced_hex_color::hex_color;
use iced_hex_color::hex_color_core;

#[test]
fn rgb_works() {
    let macro_color = hex_color!(#43ff64);
    let ref_color = iced::Color::from_rgba8(67, 255, 100, 1.0);

    dbg!(macro_color);
    dbg!(ref_color);
    assert_eq!(macro_color, ref_color)
}

#[test]
fn rbga_works() {
    let macro_color = hex_color!(#43ff64d9);
    let ref_color = iced::Color::from_rgba8(67, 255, 100, 0.85);

    dbg!(macro_color);
    dbg!(ref_color);

    assert_eq!(macro_color.r, ref_color.r);
    assert_eq!(macro_color.g, ref_color.g);
    assert_eq!(macro_color.b, ref_color.b);

    let macro_alpha = (macro_color.a * 100.0).trunc() / 100.0;
    let ref_alpha = (ref_color.a * 100.0).trunc() / 100.0;
    assert_eq!(macro_alpha, ref_alpha);
}

#[test]
fn rgb_core_works() {
    let macro_color = hex_color_core!(#43ff64);
    let ref_color = iced_core::Color::from_rgba8(67, 255, 100, 1.0);

    dbg!(macro_color);
    dbg!(ref_color);
    assert_eq!(macro_color, ref_color)
}

#[test]
fn rbga_core_works() {
    let macro_color = hex_color_core!(#43ff64d9);
    let ref_color = iced_core::Color::from_rgba8(67, 255, 100, 0.85);

    dbg!(macro_color);
    dbg!(ref_color);

    assert_eq!(macro_color.r, ref_color.r);
    assert_eq!(macro_color.g, ref_color.g);
    assert_eq!(macro_color.b, ref_color.b);

    let macro_alpha = (macro_color.a * 100.0).trunc() / 100.0;
    let ref_alpha = (ref_color.a * 100.0).trunc() / 100.0;
    assert_eq!(macro_alpha, ref_alpha);
}
