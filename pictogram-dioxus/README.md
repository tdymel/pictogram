# Pictogram dioxus
An adapter to render [pictogram](https://crates.io/crates/pictogram) icons with dioxus 0.7.

## Features
* **Drawn correctly**: The icon keeps its `fill`, `stroke`, ... and follows the text color (`currentColor`).
* **Unstyled**: A white canvas, ready to be used!
* **Custom SVG**: Use your own SVGs.
* **Lean**: It only depends on `pictogram-core` and dioxus. The icons come from your own dependency on `pictogram`.

## How to use it
```toml
[dependencies]
pictogram = "*"
pictogram-dioxus = "*"
```

```rust,ignore
rsx! {
    Pictogram {
        icon: pictogram::lucide::house::outlined,
        width: "3rem",
        height: "3rem",
        color: "red",
        ... other attributes of your liking ...
    }
}
```

### Attributes
`width`, `height`, `color`, `stroke`, ... are css properties in dioxus and win over the attributes of the icon.
So `stroke: "red"` colors an outline icon, while `color: "red"` colors every icon that uses `currentColor`.

### Provide defaults
```rust,ignore
rsx! {
    IconProvider {
        width: "1.5rem",
        height: "1.5rem",
        Pictogram { icon: pictogram::lucide::house::outlined }
    }
}
```
Attributes of the `IconProvider` replace those of the icon; attributes of the `Pictogram` replace those of the provider.

### Combining components
```rust,ignore
rsx! {
    Pictogram {
        icon: pictogram::lucide::house::outlined,
        Pictogram {
            icon: pictogram::lucide::arrow_up::outlined,
            width: 8,
            height: 8,
        }
    }
}
```

### Prepared icons
```rust,ignore
// Define icons locally - from the catalogue
define_icon!(pictogram::lucide::house::outlined);
// Or from your local assets
define_icon!(CustomIcon, "local-path-to-custom-icon.svg");

#[component]
fn SomeComponent() -> Element {
    rsx! {
        HouseOutlined { width: "3rem", height: "3rem" }
        CustomIcon { width: "3rem", height: "3rem" }
    }
}
```

## Other frameworks
`pictogram-core` has no dependencies. An icon is `Svg { view_box, attrs, body }`, and `svg.attributes()` iterates over the attributes of the root element, so an adapter for another framework is a few lines.
