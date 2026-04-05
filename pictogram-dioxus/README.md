# Pictogram dioxus
This library provides a convenient interface to create **unstyled** icons using [pictogram](https://crates.io/crates/pictogram).

## Features
* **Unstyled**: A white canvas, ready to be used!
* **Custom SVG**: Use your own SVGs.
* **Many icons**: Mix and match from any icon libraries that are supported by pictogram.

## How to use it
```toml
[dependencies]
# By default all features are enabled
pictogram = { version = "0.1", features=["material"] }
pictogram_dioxus = "0.1"
```

```rust
rsx! {
    Icon {
        icon: pictogram::svg!(pictogramm::material::action_123::filled),
        width: 48,
        height: 48,
        ... other attributes of your liking ...
    }
}
```

## License
This project is licensed under either

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](LICENSE-MIT))

at your option.