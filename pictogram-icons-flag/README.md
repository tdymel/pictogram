# Pictogram Icons Flag
Flags of countries, regions and organizations from [Flag Icons](https://github.com/lipis/flag-icons).

Every icon is a `const` [`Svg`](https://docs.rs/pictogram-core) at `<code>::<variant>`.
The compiler completes and checks the path, and only the flags you use end up in your binary.

```rust
use pictogram_icons_flag::de;

const GERMANY: pictogram_core::Svg = de::landscape;
assert_eq!(GERMANY.view_box, "0 0 640 480");

// Codes that are keywords in rust are raw identifiers: India, the Dominican Republic
let _ = pictogram_icons_flag::r#in::square;
let _ = pictogram_icons_flag::r#do::square;
```

Usually you use it through [pictogram](https://crates.io/crates/pictogram): `pictogram::flag::de::landscape`.

## Names
The names are the [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) codes in lower case (`de`, `us`).
A few flags are not countries: regions (`gb_sct`, `es_ct`), organizations (`eu`, `un`, `asean`) and `xx` (unknown).

## Variants
| Variant     | Is              |
| ----------- | --------------- |
| `landscape` | 4:3 (`4x3/`)    |
| `square`    | 1:1 (`1x1/`)    |

Flags keep their colors.

## Updates
`src/lib.rs` is generated from a release of the upstream project by `cargo xtask update flag` and is updated automatically.
The release it was generated from is `upstream-version` in `Cargo.toml`.

## License
* Code is MIT or Apache-2.0
* Icons are under the original license: [MIT](https://github.com/lipis/flag-icons/blob/main/LICENSE)
