# Pictogram Icons Simple
Icons from [Simple](https://github.com/simple-icons/simple-icons).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<variant>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_simple::github;

const ICON: pictogram_core::Svg = github::regular;
assert!(!ICON.view_box.is_empty());
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::simple::github::regular`.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update simple` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/)
