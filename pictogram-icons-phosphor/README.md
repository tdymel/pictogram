# Pictogram Icons Phosphor
Icons from [Phosphor](https://github.com/phosphor-icons/core), a flexible icon family.

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<weight>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_phosphor::house;

const HOME: pictogram_core::Svg = house::regular;
assert_eq!(HOME.view_box, "0 0 256 256");
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::phosphor::house::regular`.

## Weights
Every icon comes in all six weights: `thin`, `light`, `regular`, `bold`, `fill` and `duotone`.
The duotone weight draws a translucent second layer.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update phosphor` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: [MIT](https://github.com/phosphor-icons/core/blob/main/LICENSE)
