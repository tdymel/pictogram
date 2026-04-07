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
pictogram = { version = "*", features=["material"] }
pictogram_dioxus = "*"
```

```rust,ignore
rsx! {
    Icon {
        icon: pictogram::svg!(pictogram::material::action_123::filled),
        width: "3rem",
        height: "3rem",
        ... other attributes of your liking ...
    }
}
```

### Combining components
```rust,ignore
rsx! {
    Icon {
        icon: pictogram::svg!(pictogram::material::image_crop_free::outlined),
        height: "3rem",
        width: "3rem",
        Icon {
            icon: pictogram::svg!(pictogram::material::social_person::filled),
            height: 16,
            width: 16,
            x: 4,
            y: 4
        }
    }
}
```

### Define dedicated icon components
Best used by defining all icons in a separate `icons.rs` file.

```rust,ignore
define_icon!(pictogram::material::image_crop_free::outlined);
define_icon!(CustomIcon, "local-path-to-custom-icon.svg");

#[component]
fn SomeComponent() -> Element {
    rsx! {
        ImageCropFreeOutlined {
            height: "3rem",
            width: "3rem"
        }
        CustomIcon {
            height: "3rem",
            width: "3rem"
        }
    }
}
```

**Question**: Why dont I provide all icons predefined?
**Answer**: It increases the compile times significantly

### Provide global styling
```rust,ignore
IconProvider {
    height: "3rem",
    width: "3rem",
    fill: "blue",
    Icon {
        icon: pictogram::svg!(pictogram::material::image_crop_free::outlined),
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