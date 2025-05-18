# Plist Plus

A safe Rust wrapper around [libplist](https://github.com/libimobiledevice/libplist).

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

### Reading a plist

The following example is taken from `idevicelistapps.rs` of `rusty_libimobiledevice`
and was simplified. It looks up all the installed apps on an iOS device.

```rust
use plist_plus::{Array, Dictionary, Value};

fn lookup<'a>() -> Value<'a> {
    let arr = Value::Array(Array::new());
    /* some code is omitted */
    return arr;
}

// Returns all installed apps as a plist
let lookup_results_plist: Value = lookup();
// The returned value is actually an Array containing Dictionaries
let lookup_results: Array = lookup_results_plist.into_array().unwrap();

// Interate over it and print values from a Dictionary
for item in &lookup_results {
    let dict: &Dictionary = item.as_dictionary().unwrap();
    let app_id = dict.get("CFBundleIdentifier").unwrap().as_string().unwrap().to_string();
    let app_name = dict.get("CFBundleDisplayName").unwrap().as_string().unwrap().to_string();
    println!("{app_id}: {app_name}");
}
```

## Parsing a plist from a file

```rust
use plist_plus::{from_file, Dictionary};

// Read a plist containing a dictionary
let plist: Dictionary = from_file("./tests/book.plist").unwrap().into_dictionary().unwrap();
// Print its keys and values
for (key, value) in &plist {
    println!("{} => {:?}", key, value)
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
