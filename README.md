# Plist Plus 2

A safe Rust wrapper around [libplist](https://github.com/libimobiledevice/libplist).
Based on the [jkcoxson's repo](https://github.com/jkcoxson/plist_plus).

Use this crate when dealing with C libraries or Rust FFI wrappers which use `libplist`.
In other cases [plist](https://crates.io/crates/plist) crate is more convenient and efficient.

The crate follows the API style used in the [plist](https://crates.io/crates/plist) crate.

The crate currently targets libplist v2.7.0.

## Notes

Every child node of an array or dictionary is tied to a lifetime of its parent.
Since the crate is a wrapper of the C library you can't actually *own* any children
(only clone them if needed). For instance, using `Array<'a>::get` method returns
an `Item<'a>` that only can be access while the original array exists.

## Examples

### Parsing a plist from a file

```rust
use plist_plus::{from_file, Dictionary};

// Read a plist containing a dictionary
let plist: Dictionary = from_file("./tests/book.plist").unwrap().into_dictionary().unwrap();
// Print its keys and values
for (key, value) in &plist {
    println!("{key} => {value:?}");
}
```

### Creating a plist

```rust
fn main() -> Result<(), plist_plus::Error> {
    use plist_plus::{array, Dictionary, dict, Node};

    // Create a basic plist
    let plist: Dictionary = dict!(
        "First key" => "hello world",
        "Second key" => 123,
        "Third key" => array!("APT.", 2.50)
    );

    // Export it as an XML plist string
    let xml: String = plist.to_xml()?;
    println!("{xml}");
    Ok(())
}
```

## Crate features

* `pls-generate`: regenerates FFI bindings. Use carefully, since it pulls the latest commit from the `libplist` repository and it may introduce API break changes.

* `vendored`: clones and builds `libplist` to be included in the binary. It uses the specified tag (version) that the crate targets. Combining it with `pls-generate` will result in using the latest version of the library.

* `clean_debug`: enables clean debug behavior that prints the actual inner values of any plist node. Enabled by default. Disable it for viewing pointers and other stuff (useful for debugging).
