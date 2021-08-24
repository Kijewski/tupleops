Ensure that two types are the same, or fail with a compilation error.

```rust
use same_types::assert_same_types;

assert_same_types!(u32, u32, u32, u32);
```

```rust
use same_types::assert_same_types;

// Fails with the message:
// the trait `SameTypes` is not implemented for `(i32, u32)`
assert_same_types!(u32, u32, i32, u32);
```
