# Pictogram Icons Hero
Icons from [heroicons](https://github.com/tailwindlabs/heroicons).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<variant>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_hero::bell;

const ICON: pictogram_core::Svg = bell::outlined;
assert!(!ICON.view_box.is_empty());
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::hero::bell::outlined`.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update hero` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: MIT
