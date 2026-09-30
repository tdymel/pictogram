# Pictogram core
The framework independent data model of [pictogram](https://crates.io/crates/pictogram).
It has **no dependencies** and is `no_std`.

An icon is a plain `const` value:

```rust
use pictogram_core::Svg;

const CIRCLE: Svg = Svg::new(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor">
  <circle cx="12" cy="12" r="10" />
</svg>"#);

assert_eq!(CIRCLE.view_box, "0 0 24 24");
assert_eq!(CIRCLE.attributes().count(), 2); // fill, stroke
```

* `view_box`: the `viewBox` of the icon.
* `attrs`: the presentation attributes of the root element (`fill`, `stroke`, ...).
  Renderers apply them so an icon is drawn the way its author intended.
* `body`: everything inside the root element.

`Svg::new` runs at compile time when used in a `const`, so nothing is parsed at runtime.
