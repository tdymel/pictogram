# Pictogram Icons Fluent Emoji
Emoji from [Fluent Emoji](https://github.com/microsoft/fluentui-emoji) by Microsoft.

Every emoji is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<emoji name>::<variant>`.
The compiler completes and checks the path, and only the emoji you use end up in your binary.

```rust
use pictogram_icons_fluent_emoji::thumbs_up;

const THUMBS_UP: pictogram_core::Svg = thumbs_up::flat;
assert_eq!(THUMBS_UP.view_box, "0 0 32 32");

// Names that start with a digit get a `_`
let _ = pictogram_icons_fluent_emoji::_1st_place_medal::flat;
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::fluent_emoji::thumbs_up::flat`.

## Names
The names are the [CLDR](https://cldr.unicode.org/) names of the emoji, as upstream names its files: `grinning_face`, `red_heart`, `e-mail` (`e_mail`).

## Variants
| Variant         | Is                                        |
| --------------- | ----------------------------------------- |
| `flat`          | in color                                  |
| `high_contrast` | in one color, which follows the text color |

Every emoji has both variants. Upstream also has `Color` (with gradients) and `3D` (png) styles and skin tones,
which are not included: the colored emoji alone are 40 MB of svg. Emoji with skin tones are the default (yellow) one.

## Updates
Fluent Emoji has no releases. `src/lib.rs` is generated from the tip of its default branch by `cargo xtask update fluent-emoji` and is updated automatically.
The commit it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Emoji are under the original license: [MIT](https://github.com/microsoft/fluentui-emoji/blob/main/LICENSE)
