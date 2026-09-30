# Pictogram Icons Lucide
Icons from [lucide](https://github.com/lucide-icons/lucide).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::outlined`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_lucide::house;

const HOME: pictogram_core::Svg = house::outlined;
assert_eq!(HOME.view_box, "0 0 24 24");

// Names that are keywords in rust are raw identifiers
let _ = pictogram_icons_lucide::r#box::outlined;
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::lucide::house::outlined`.

## Updates
`src/lib.rs` is generated from a release of lucide by `cargo xtask update lucide` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: ISC
