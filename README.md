# Borrowed-Or-oWned

Provide a Borrowed-Or-oWned smart pointer.

Alternative to `Cow` for which the `Clone` trait is not required for
the encapsulated type.

Use this crate if you want something like `Cow` but your type cannot be
cloned.

You can find the rustdoc [here](http://boussejra.com/rust-docs/boow/boow).

# How to use

```rust
extern crate boow;
use boow::Bow;
// This struct contains a type for which we cannot know at compile time
// whether it will be owned or borrowed.
struct MyStruct<'a> {
    borrowed_or_owned: Bow<'a, InnerStruct>,
}
struct InnerStruct {
    _stuff: String,
}
impl<'a> MyStruct<'a> {
    // Use borrowed value
    fn from_borrowed(inner: &'a InnerStruct) -> Self {
        Self { borrowed_or_owned: Bow::Borrowed(inner) }
    }
    // Use owned value
    fn from_owned(inner: InnerStruct) -> Self {
        Self { borrowed_or_owned: Bow::Owned(inner) }
    }
}
```
