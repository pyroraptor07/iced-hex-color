# iced_hex_color
This crate contains a few utility macros for generating Iced color structs
from standard hex color codes.

- **[hex_color!]**: Generates code for an [iced::Color](https://docs.rs/iced/latest/iced/struct.Color.html) struct
- **[hex_color_core!]**: Generates code for an [iced_core::Color](https://docs.rs/iced_core/latest/iced_core/struct.Color.html) struct

See the [hex_color!] macro for usage details; aside from the Iced crate used, both macros are identical.

This crate is compatible with the Iced version 0.10.

[hex_color!]: https://docs.rs/iced-hex-color/latest/iced_hex_color/macro.hex_color.html
[hex_color_core!]: https://docs.rs/iced-hex-color/latest/iced_hex_color/macro.hex_color_core.html
