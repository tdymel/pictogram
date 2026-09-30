# Pictogram Icons Tabler
Icons from [tabler](https://github.com/tabler/tabler-icons).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<variant>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_tabler::home;

const ICON: pictogram_core::Svg = home::outlined;
assert!(!ICON.view_box.is_empty());
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::tabler::home::outlined`.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update tabler` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: MIT
