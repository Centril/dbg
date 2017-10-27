# [RFC 2173]: The `dbg!(expr, ..)` macro

This crate provides a working implementation of [RFC 2173] on stable Rust.

To use the crate, add this to `Cargo.toml`:

```rust
[dependencies]
dbg = "1.0.3"
```

and to `lib.rs` or `main.rs` (the crate root):

```rust
#[macro_use] extern crate dbg;
```

Now you are ready to use the macro. Happy debugging!

If you are on nightly, the macro will automatically be built with the
[specialization features] mandated by the RFC.

For more details and documentation, please see the [guide-level explanation].

[guide-level explanation]: https://github.com/Centril/rfcs/blob/rfc/quick-debug-macro/text/0000-quick-debug-macro.md#guide-level-explanation

[RFC 2173]: https://github.com/rust-lang/rfcs/pull/2173

[specialization features]: https://github.com/Centril/rfcs/blob/rfc/quick-debug-macro/text/0000-quick-debug-macro.md#types-which-are-not-debug