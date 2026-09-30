# Pictogram Icons Material
Icons from [material-design-icons](https://github.com/google/material-design-icons).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<variant>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_material::action_home;

const ICON: pictogram_core::Svg = action_home::filled;
assert!(!ICON.view_box.is_empty());
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::material::action_home::filled`.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update material` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: Apache-2.0
