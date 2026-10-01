# Pictogram Icons Lobe
Icons from [Lobe Icons](https://github.com/lobehub/lobe-icons): the logos of AI models, providers and tools.

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<icon name>::<variant>`.
The compiler completes and checks the path, and only the icons you use end up in your binary.

```rust
use pictogram_icons_lobe::openai;

const ICON: pictogram_core::Svg = openai::mono;
assert!(!ICON.view_box.is_empty());
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::lobe::openai::mono`.

## Variants
| Variant       | Is                                                            |
| ------------- | ------------------------------------------------------------- |
| `mono`        | the logo in the text color                                    |
| `color`       | the logo in the colors of the brand (not every icon has one)  |
| `text`        | the wordmark, in the text color                               |
| `text_color`  | the wordmark in the colors of the brand                       |
| `text_cn`     | the chinese wordmark                                          |
| `brand`       | the logo together with the name of the product, in the text color |
| `brand_color` | the same in the colors of the brand                           |

Not every icon has every variant, the compiler tells you.
The `color` variants keep their own colors, the others follow the text color (a few, like `crusoe::mono`, hard code one upstream).

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update lobe` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: [MIT](https://github.com/lobehub/lobe-icons/blob/master/LICENSE)
* The logos are trademarks of their owners.
