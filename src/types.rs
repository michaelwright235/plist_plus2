pub mod array;
mod boolean;
mod data;
mod date;
pub mod dictionary;
mod integer;
mod key;
mod null;
mod real;
mod string;
mod uid;

pub use array::Array;
pub use boolean::Boolean;
pub use data::Data;
pub use date::Date;
pub use dictionary::Dictionary;
pub use integer::Integer;
pub use key::Key;
pub use null::Null;
pub use real::Real;
pub use string::PString;
pub use uid::Uid;

use crate::{unsafe_bindings::{self, plist_type}, Error, Value, PLIST_ERROR_SUCCESS};

/// A common trait for any plist node.
pub trait Node: crate::plist_ffi::PlistFFI {
    /// Exports the plist node as an XML format.
    fn to_xml(&self) -> Result<String, Error> {
        let mut xml_ptr = std::ptr::null_mut();
        let mut xml_size = 0;
        let result =
            unsafe { unsafe_bindings::plist_to_xml(self.pointer(), &mut xml_ptr, &mut xml_size) };

        if result != PLIST_ERROR_SUCCESS {
            return Err(result.into());
        }

        let xml_slice =
            unsafe { std::slice::from_raw_parts(xml_ptr as *mut u8, xml_size as usize) };
        let xml_string = std::str::from_utf8(xml_slice)
            .map_err(|_| Error::Unknown)?
            .to_string();

        unsafe { unsafe_bindings::plist_mem_free(xml_ptr as *mut _) };

        Ok(xml_string)
    }

    /// Exports the plist node as a JSON format.
    ///
    /// Set `prettify` to `true` to compose a prettified JSON string.
    fn to_json(&self, prettify: bool) -> Result<String, Error> {
        let mut json_ptr = std::ptr::null_mut();
        let mut json_size = 0;
        let result = unsafe {
            unsafe_bindings::plist_to_json(
                self.pointer(),
                &mut json_ptr,
                &mut json_size,
                prettify as _,
            )
        };
        if result != PLIST_ERROR_SUCCESS {
            return Err(result.into());
        }

        let json_slice =
            unsafe { std::slice::from_raw_parts(json_ptr as *mut u8, json_size as usize) };
        let json_string = std::str::from_utf8(json_slice)
            .map_err(|_| Error::Unknown)?
            .to_string();

        // Free the allocated memory
        unsafe { unsafe_bindings::plist_mem_free(json_ptr as *mut _) };

        Ok(json_string)
    }

    /// Exports the plist node as a binary encoded plist.
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes_ptr = std::ptr::null_mut();
        let mut bytes_size = 0;
        let result = unsafe {
            unsafe_bindings::plist_to_bin(self.pointer(), &mut bytes_ptr, &mut bytes_size)
        };

        if result != PLIST_ERROR_SUCCESS {
            return Err(result.into());
        }

        let bytes_vec =
            unsafe { std::slice::from_raw_parts(bytes_ptr as *const u8, bytes_size as usize) }
                .to_vec();

        // Free the allocated memory
        unsafe { unsafe_bindings::plist_mem_free(bytes_ptr as *mut _) };

        Ok(bytes_vec)
    }

    /// Exports the plist node to an OpenStep ASCII encoded plist.
    ///
    /// Set `prettify` to `true` to compose a prettified string.
    fn to_openstep(&self, prettify: bool) -> Result<String, Error> {
        let mut openstep_ptr = std::ptr::null_mut();
        let mut openstep_size = 0;
        let result = unsafe {
            unsafe_bindings::plist_to_openstep(
                self.pointer(),
                &mut openstep_ptr,
                &mut openstep_size,
                prettify as _,
            )
        };
        if result != PLIST_ERROR_SUCCESS {
            return Err(result.into());
        }

        let openstep_slice =
            unsafe { std::slice::from_raw_parts(openstep_ptr as *mut u8, openstep_size as usize) };
        let openstep_string = std::str::from_utf8(openstep_slice)
            .map_err(|_| Error::Unknown)?
            .to_string();

        // Free the allocated memory
        unsafe { unsafe_bindings::plist_mem_free(openstep_ptr as *mut _) };

        Ok(openstep_string)
    }
}

/// The type of a given plist
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NodeType {
    Boolean,
    Integer,
    Real,
    Date,
    Data,
    String,
    Array,
    Dictionary,
    Key,
    Uid,
    Null,
}

impl From<plist_type> for NodeType {
    fn from(i: plist_type) -> Self {
        match i {
            unsafe_bindings::plist_type_PLIST_NONE => {
                panic!("`None` variant shoudn't happen. An invalid pointer has been passed.")
            }
            unsafe_bindings::plist_type_PLIST_BOOLEAN => NodeType::Boolean,
            unsafe_bindings::plist_type_PLIST_INT => NodeType::Integer,
            unsafe_bindings::plist_type_PLIST_REAL => NodeType::Real,
            unsafe_bindings::plist_type_PLIST_STRING => NodeType::String,
            unsafe_bindings::plist_type_PLIST_ARRAY => NodeType::Array,
            unsafe_bindings::plist_type_PLIST_DICT => NodeType::Dictionary,
            unsafe_bindings::plist_type_PLIST_DATE => NodeType::Date,
            unsafe_bindings::plist_type_PLIST_DATA => NodeType::Data,
            unsafe_bindings::plist_type_PLIST_KEY => NodeType::Key,
            unsafe_bindings::plist_type_PLIST_UID => NodeType::Uid,
            unsafe_bindings::plist_type_PLIST_NULL => NodeType::Null,
            _ => panic!("Unknown plist type"),
        }
    }
}

// The main reason of introducing a separate struct to contain a value is
// that returning a plain Value results in breaking Rust ownership rules. For instance,
// you can have an *immutable* Array and get a Value: now you do *whatever* you want with it.
// Item and ItemMut resolves this problem by dereferencing itself to an immutable
// or mutable reference to a value.

/// Represents an immutable referenced array/dictionary item.
///
/// It automatically dereferences to the underlying [Value].
#[derive(Debug, PartialEq)]
pub struct Item<'a>(Value<'a>);

impl<'a> std::ops::Deref for Item<'a> {
    type Target = Value<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a mutable referenced array/dictionary item.
///
/// It automatically dereferences to the underlying [Value].
///
/// Since we're dealing with a C library, please use [Value::replace_with]
/// instead of dereferencing an Item and assinging a new one. It will
/// actually do nothing.
///
/// ```rust
/// use plist_plus2::{array, Boolean, Value};
///
/// let mut arr = array!(0, 1, 2);
/// let new_value: Value = Boolean::new(true).into();
///
/// // *arr.get_mut(0).unwrap() = new_value; // Don't do this
/// arr.get_mut(0).unwrap().replace_with(&new_value); // Do this instead
///
/// let updated_value = arr.get(0).unwrap();
/// assert_eq!(updated_value.as_boolean(), new_value.as_boolean())
/// ```
#[derive(Debug, PartialEq)]
pub struct ItemMut<'a>(Value<'a>);

impl<'a> std::ops::Deref for ItemMut<'a> {
    type Target = Value<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// An internal marco for automatic implementation of any plist node.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_node {
    ($(#[$outer:meta])* $name:ident) => {
        $(#[$outer])*
        #[cfg_attr(not(feature = "clean_debug"), derive(Debug))]
        pub struct $name<'a> {
            pub(crate) pointer: $crate::unsafe_bindings::plist_t,
            pub(crate) false_drop: bool,
            // Used if the current plist is a child
            pub(crate) phantom: std::marker::PhantomData<&'a $crate::$name<'a>>,
        }

        impl $crate::plist_ffi::PlistFFI for $name<'_> {
            fn pointer(&self) -> $crate::unsafe_bindings::plist_t {
                self.pointer
            }

            fn false_drop(&self) -> bool {
                self.false_drop
            }

            fn set_false_drop(&mut self, value: bool) {
                self.false_drop = value
            }
        }

        impl $crate::Node for $name<'_> {}

        impl<'a> From<$name<'a>> for $crate::Value<'a> {
            fn from(value: $name<'a>) -> Self {
                $crate::Value::$name(value)
            }
        }

        impl Drop for $name<'_> {
            fn drop(&mut self) {
                use $crate::plist_ffi::PlistFFI;
                if !self.false_drop() {
                    unsafe { $crate::unsafe_bindings::plist_free(self.pointer()) }
                }
            }
        }
    };
}
