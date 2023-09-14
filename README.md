# atomic\_prim\_traits

[![License](https://img.shields.io/crates/l/atomic_prim_traits.svg)](https://github.com/irrustible/atomic_prim_traits/blob/main/LICENSE)
[![Package](https://img.shields.io/crates/v/atomic_prim_traits.svg)](https://crates.io/crates/atomic_prim_traits)
[![Documentation](https://docs.rs/atomic_prim_traits/badge.svg)](https://docs.rs/atomic_prim_traits)

Traits over atomic primitive integer types.

## Example

```rust
fn incr<T: AtomicInt>(value: &T) -> Result<<T as AtomicInt>::Prim, <T as AtomicInt>::Prim> {
    value.fetch_update(
        Ordering::SeqCst,
        Ordering::SeqCst,
        |i| Some(if i == T::MAX { T::MIN } else { i + T::ONE })
    )
}

let value = AtomicU8::new(255);
assert_eq!(incr(&value), Ok(255));
assert_eq!(incr(&value), Ok(0));
```

## Notes

* Enable feature `nightly` to get `as_mut_ptr` when you have a nightly compiler available.
* Rust 1.45.0 or newer is required.

## Copyright and License

    Copyright (c) 2020-2023 James Laver, atomic_prim_traits contributors.

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.
