# Pictogram
Pictogram resolves icons at compile time using data stored in the manifest directory. 
During compilation an icon is looked up and baked into your application.
This bypasses compilation penalties as the entire icon data does not have to be compiled by rustc.

## Features
* **NOT** vibe coded. 
* **Type-Safe**: If an icon can't be resolved, it will lead to a compilation error.
* **Compile-Time**: Icons are baked into your code at compile time.
* **Only pay for what you need**: Only compile the icons, you actually use.
* **Framework independent**: Although, there is an [adapter for dioxus](https://crates.io/crates/pictogram-dioxus). 

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
rsx! {
    Icon {
        icon: pictogram::svg!(pictogram::material::action_123::filled),
        width: 48,
        height: 48,
        ... other attributes of your liking ...
    }
}
```

## Supported libraries
More libraries are going to be added soon.

| Library                 | Feature   | Crate |
| ----------------------- | --------- | ----- |
| Material design icons   | material  | [pictogram-icons-material](https://crates.io/crates/pictogram-icons-material) |
| Bootstrap               | bootstrap | [pictogram-icons-bootstrap](https://crates.io/crates/pictogram-icons-bootstrap) |
| Feather                 | feather   | [pictogram-icons-feather](https://crates.io/crates/pictogram-icons-feather) |

## License
This project is licensed under either

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.