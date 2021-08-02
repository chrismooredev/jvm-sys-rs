
# jvm-sys

Bindgen-generated bindings for `jni.h`, `jawt.h`, and/or `jvmti.h`, as distributed within a Java distribution.

## Features:
`link`: (default) Enables linking to each enabled library
`jni`: (default) Enables generating bindings for, and linking to (if enabled), the system Java Native Interface library.
`jawt`: Enables generating bindings for, and linking to (if enabled), the system Java Abstract Window Toolkit library.
`jvmti`: Enables generating bindings for, and linking to (if enabled), the system Java Virtual Machine Tools Interface library.

Please note that `jawt` and `jvmti` bindings are experimental, and should be considered untested. Confirmation that these work would be welcome in the form of issues/etc.

## Usage
You should just be able to pull the crate in as a dependency, and add the following to the consuming crate's `lib.rs`/`main.rs` file. (If you don't, a linkage error may occur).
```rust
extern crate jvm_sys;
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
