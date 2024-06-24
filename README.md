# Selthi

[![Latest Version]][crates.io] [![Docs]][docs.rs] ![License]

[crates.io]: https://crates.io/crates/selthi
[latest version]: https://img.shields.io/crates/v/selthi.svg
[docs]: https://img.shields.io/docsrs/selthi/latest?logo=docs.rs
[docs.rs]: https://docs.rs/selthi
[license]: https://img.shields.io/crates/l/selthi.svg

[selthi](https://github.com/anotherlusitano/selthi) is a library for building interactive prompts with or without images, inspired by [inquire](https://github.com/mikaelmello/inquire).

It provides a prompt asking the user to select one option from a given list, with the ability to display images for each option

## Demo

![Animated GIF making a demonstration of this library](./assets/images.gif)

[Source](./examples/images.rs)

## Examples

Examples can be found in the `examples` directory. Run them to see basic behavior:

```
cargo run --example images --features with_images
```

## Usage

Add Selthi to your dependencies.

```
cargo add selthi
```

\* If you want to support images, add the feature `with_images`

```
selthi = { version = "0.1.3", features = ["with_images"] }
```
