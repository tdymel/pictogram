# Pictogram Icons Iconoir
Icons from [Iconoir](https://github.com/iconoir-icons/iconoir).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<variant>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_iconoir::home;

const HOME: pictogram_core::Svg = home::regular;
assert_eq!(HOME.view_box, "0 0 24 24");

// Names that are keywords in rust are raw identifiers
let _ = pictogram_icons_iconoir::r#box::regular;
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::iconoir::home::regular`.

## Variants
Every icon is available as `regular` (drawn with strokes). Some also come as `solid` (filled), the compiler tells you.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update iconoir` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: [MIT](https://github.com/iconoir-icons/iconoir/blob/main/LICENSE)
