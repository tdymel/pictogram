# Pictogram
Pictogram resolves icons at compile time using data stored in the manifest directory. 
During compilation an icon is looked up and baked into your application.
This bypasses compilation penalties as the entire icon data does not have to be compiled by rustc.

## Features
* **Support for many libraries**: See supported libraries for more information.
* **Type-Safe**: If an icon can't be resolved, it will lead to a compilation error.
* **Compile-Time**: Icons are baked into your code at compile time.
* **Only pay for what you need**: Only compile the icons, you actually use.
* **Framework independent**: Although, there is an [adapter for dioxus](https://crates.io/crates/pictogram-dioxus). 
* **NOT** vibe coded. 

## How to use it
```toml
# By default all features are enabled
pictogram = { version = "*", features=["material"] }
```
```rust,ignore
let svg = pictogram::svg!(pictogram::material::action_123::filled);
println!("{}", svg);
```

### Dioxus adapter
The adapter is available [here](https://crates.io/crates/pictogram-dioxus).

```rust,ignore
// Define icons locally - from the index
define_icon!(pictogram::material::social_person::filled);
// Or from your local assets
define_icon!(CustomIcon, "local-path-to-custom-icon.svg");

#[component]
fn SomeComponent() -> Element {
  rsx! {
      // Or use the general component
      Icon {
          icon: pictogram::svg!(pictogram::material::image_crop_free::outlined),
          width: 48,
          height: 48,
          // Compose icons
          SocialPersonFilled {
            height: 16,
            width: 16,
            x: 4,
            y: 4
        }
      }
      CustomIcon {
        width: 48,
        height: 48
      }
  }
}
```

## Supported libraries
More libraries are going to be added soon.

| Library                 | Feature      | License    | Crate |
| ----------------------- | ------------ | ---------- | ----- |
| Material design icons   | material     | Apache-2.0 | [material](https://crates.io/crates/pictogram-icons-material) |
| Bootstrap               | bootstrap    | MIT        | [bootstrap](https://crates.io/crates/pictogram-icons-bootstrap) |
| Feather                 | feather      | MIT        | [feather](https://crates.io/crates/pictogram-icons-feather) |
| Font Awesome            | font-awesome | CC BY 4.0  | [font-awesome](https://crates.io/crates/pictogram-icons-font-awesome) |
| Tabler                  | tabler       | MIT        | [tabler](https://crates.io/crates/pictogram-icons-tabler) |
| Simple                  | simple       | CC0-1.0    | [simple](https://crates.io/crates/pictogram-icons-simple) |
| Heroicons               | hero         | MIT        | [hero](https://crates.io/crates/pictogram-icons-hero) |

### TODO List
* Ion icons: https://github.com/ionic-team/ionicons
* Lucide: https://github.com/lucide-icons/lucide
* Octicons: https://github.com/primer/octicons
* Vscode Codicons: https://github.com/microsoft/vscode-codicons

## License
This project is licensed under either

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Icon Licenses
All icons are licensed under their respective licenses. 
Please contact me, if any repository would like to be removed/updated.