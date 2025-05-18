#![doc = include_str!("../README.md")]

mod error;
mod types;
mod unsafe_bindings;
pub use error::*;
pub use types::*;

use std::ffi::CString;

/// Represents any plist value.
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Array(Array<'a>),
    Boolean(Boolean<'a>),
    Data(Data<'a>),
    Date(Date<'a>),
    Dictionary(Dictionary<'a>),
    Integer(Integer<'a>),
    Key(Key<'a>),
    Null(Null<'a>),
    Real(Real<'a>),
    PString(PString<'a>),
    Uid(Uid<'a>),
}

impl<'a> Value<'a> {
    /// Exports the plist node as an XML format.
    pub fn to_xml(&self) -> Result<String, Error> {
        self.as_node().to_xml()
    }

    /// Exports the plist node as a JSON format.
    ///
    /// Set `prettify` to `true` to compose a prettified JSON string.
    pub fn to_json(&self, prettify: bool) -> Result<String, Error> {
        self.as_node().to_json(prettify)
    }

    /// Exports the plist node as a binary encoded plist.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        self.as_node().to_bytes()
    }

    /// Exports the plist node to an OpenStep ASCII encoded plist.
    ///
    /// Set `prettify` to `true` to compose a prettified string.
    pub fn to_openstep(&self, prettify: bool) -> Result<String, Error> {
        self.as_node().to_openstep(prettify)
    }

    /// Returns a mutable reference to the value as a dynamic [Node] object.
    pub(crate) fn as_node_mut(&mut self) -> &mut dyn Node {
        match self {
            Value::Array(v) => v as &mut dyn Node,
            Value::Boolean(v) => v as &mut dyn Node,
            Value::Data(v) => v as &mut dyn Node,
            Value::Date(v) => v as &mut dyn Node,
            Value::Dictionary(v) => v as &mut dyn Node,
            Value::Integer(v) => v as &mut dyn Node,
            Value::Key(v) => v as &mut dyn Node,
            Value::Real(v) => v as &mut dyn Node,
            Value::PString(v) => v as &mut dyn Node,
            Value::Uid(v) => v as &mut dyn Node,
            Value::Null(v) => v as &mut dyn Node,
        }
    }

    /// Returns an immutable reference to the value as a dynamic [Node] object.
    pub(crate) fn as_node(&self) -> &dyn Node {
        match self {
            Value::Array(v) => v as &dyn Node,
            Value::Boolean(v) => v as &dyn Node,
            Value::Data(v) => v as &dyn Node,
            Value::Date(v) => v as &dyn Node,
            Value::Dictionary(v) => v as &dyn Node,
            Value::Integer(v) => v as &dyn Node,
            Value::Key(v) => v as &dyn Node,
            Value::Real(v) => v as &dyn Node,
            Value::PString(v) => v as &dyn Node,
            Value::Uid(v) => v as &dyn Node,
            Value::Null(v) => v as &dyn Node,
        }
    }

    /// If the [Value] is an Array, returns an immutable reference to the associated [Array].
    ///
    /// Returns [None] otherwise.
    pub fn as_array(&self) -> Option<&Array<'a>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    /// If the [Value] is an Array, returns a mutable reference to the associated [Array].
    ///
    /// Returns [None] otherwise.
    pub fn as_array_mut(&mut self) -> Option<&mut Array<'a>> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    /// If the [Value] is a Boolean, returns an immutable reference to the associated [Boolean].
    ///
    /// Returns [None] otherwise.
    pub fn as_boolean(&self) -> Option<&Boolean<'a>> {
        match self {
            Value::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    /// If the [Value] is a Boolean, returns a mutable reference to the associated [Boolean].
    ///
    /// Returns [None] otherwise.
    pub fn as_boolean_mut(&mut self) -> Option<&mut Boolean<'a>> {
        match self {
            Value::Boolean(boolean) => Some(boolean),
            _ => None,
        }
    }

    /// If the [Value] is a Data, returns an immutable reference to the associated [Data].
    ///
    /// Returns [None] otherwise.
    pub fn as_data(&self) -> Option<&Data<'a>> {
        match self {
            Value::Data(data) => Some(data),
            _ => None,
        }
    }

    /// If the [Value] is a Data, returns a mutable reference to the associated [Data].
    ///
    /// Returns [None] otherwise.
    pub fn as_data_mut(&mut self) -> Option<&mut Data<'a>> {
        match self {
            Value::Data(data) => Some(data),
            _ => None,
        }
    }

    /// If the [Value] is a Date, returns an immutable reference to the associated [Date].
    ///
    /// Returns [None] otherwise.
    pub fn as_date(&self) -> Option<&Date<'a>> {
        match self {
            Value::Date(date) => Some(date),
            _ => None,
        }
    }

    /// If the [Value] is a Date, returns a mutable reference to the associated [Date].
    ///
    /// Returns [None] otherwise.
    pub fn as_date_mut(&mut self) -> Option<&mut Date<'a>> {
        match self {
            Value::Date(date) => Some(date),
            _ => None,
        }
    }

    /// If the [Value] is a Dictionary, returns an immutable reference to the associated [Dictionary].
    ///
    /// Returns [None] otherwise.
    pub fn as_dictionary(&self) -> Option<&Dictionary<'a>> {
        match self {
            Value::Dictionary(dictionary) => Some(dictionary),
            _ => None,
        }
    }

    /// If the [Value] is a Dictionary, returns a mutable reference to the associated [Dictionary].
    ///
    /// Returns [None] otherwise.
    pub fn as_dictionary_mut(&mut self) -> Option<&mut Dictionary<'a>> {
        match self {
            Value::Dictionary(dictionary) => Some(dictionary),
            _ => None,
        }
    }

    /// If the [Value] is a Real, returns an immutable reference to the associated [Real].
    ///
    /// Returns [None] otherwise.
    pub fn as_real(&self) -> Option<&Real<'a>> {
        match self {
            Value::Real(real) => Some(real),
            _ => None,
        }
    }

    /// If the [Value] is a Real, returns a mutable reference to the associated [Real].
    ///
    /// Returns [None] otherwise.
    pub fn as_real_mut(&mut self) -> Option<&mut Real<'a>> {
        match self {
            Value::Real(real) => Some(real),
            _ => None,
        }
    }

    /// If the [Value] is an Integer, returns an immutable reference to the associated [Integer].
    ///
    /// Returns [None] otherwise.
    pub fn as_integer(&self) -> Option<&Integer<'a>> {
        match self {
            Value::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    /// If the [Value] is an Integer, returns a mutable reference to the associated [Integer].
    ///
    /// Returns [None] otherwise.
    pub fn as_integer_mut(&mut self) -> Option<&mut Integer<'a>> {
        match self {
            Value::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    /// If the [Value] is a Key, returns an immutable reference to the associated [Key].
    ///
    /// Returns [None] otherwise.
    pub fn as_key(&self) -> Option<&Key<'a>> {
        match self {
            Value::Key(key) => Some(key),
            _ => None,
        }
    }

    /// If the [Value] is a Key, returns a mutable reference to the associated [Key].
    ///
    /// Returns [None] otherwise.
    pub fn as_key_mut(&mut self) -> Option<&mut Key<'a>> {
        match self {
            Value::Key(key) => Some(key),
            _ => None,
        }
    }

    /// If the [Value] is a String, returns an immutable reference to the associated [PString].
    ///
    /// Returns [None] otherwise.
    pub fn as_string(&self) -> Option<&PString<'a>> {
        match self {
            Value::PString(string) => Some(string),
            _ => None,
        }
    }

    /// If the [Value] is a String, returns a mutable reference to the associated [PString].
    ///
    /// Returns [None] otherwise.
    pub fn as_string_mut(&mut self) -> Option<&mut PString<'a>> {
        match self {
            Value::PString(string) => Some(string),
            _ => None,
        }
    }

    /// If the [Value] is a Uid, returns an immutable reference to the associated [Uid].
    ///
    /// Returns [None] otherwise.
    pub fn as_uid(&self) -> Option<&Uid<'a>> {
        match self {
            Value::Uid(uid) => Some(uid),
            _ => None,
        }
    }

    /// If the [Value] is a Uid, returns a mutable reference to the associated [Uid].
    ///
    /// Returns [None] otherwise.
    pub fn as_uid_mut(&mut self) -> Option<&mut Uid<'a>> {
        match self {
            Value::Uid(uid) => Some(uid),
            _ => None,
        }
    }

    /// If the [Value] is an Array, consumes itself and returns the associated [Array].
    ///
    /// Returns [None] otherwise.
    pub fn into_array(self) -> Option<Array<'a>> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Boolean, consumes itself and returns the associated [Boolean].
    ///
    /// Returns [None] otherwise.
    pub fn into_boolean(self) -> Option<Boolean<'a>> {
        match self {
            Value::Boolean(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Data, consumes itself and returns the associated [Data].
    ///
    /// Returns [None] otherwise.
    pub fn into_data(self) -> Option<Data<'a>> {
        match self {
            Value::Data(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Date, consumes itself and returns the associated [Date].
    ///
    /// Returns [None] otherwise.
    pub fn into_date(self) -> Option<Date<'a>> {
        match self {
            Value::Date(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Dictionary, consumes itself and returns the associated [Dictionary].
    ///
    /// Returns [None] otherwise.
    pub fn into_dictionary(self) -> Option<Dictionary<'a>> {
        match self {
            Value::Dictionary(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is an Integer, consumes itself and returns the associated [Integer].
    ///
    /// Returns [None] otherwise.
    pub fn into_integer(self) -> Option<Integer<'a>> {
        match self {
            Value::Integer(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Key, consumes itself and returns the associated [Key].
    ///
    /// Returns [None] otherwise.
    pub fn into_key(self) -> Option<Key<'a>> {
        match self {
            Value::Key(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Real, consumes itself and returns the associated [Real].
    ///
    /// Returns [None] otherwise.
    pub fn into_real(self) -> Option<Real<'a>> {
        match self {
            Value::Real(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a String, consumes itself and returns the associated [PString].
    ///
    /// Returns [None] otherwise.
    pub fn into_string(self) -> Option<PString<'a>> {
        match self {
            Value::PString(v) => Some(v),
            _ => None,
        }
    }

    /// If the [Value] is a Uid, consumes itself and returns the associated [Uid].
    ///
    /// Returns [None] otherwise.
    pub fn into_uid(self) -> Option<Uid<'a>> {
        match self {
            Value::Uid(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `true` if the [Value] is a [Null].
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null(_))
    }

    /// Replaces the current Value with another one.
    ///
    /// The `new_value` will be cloned (this is how the C library works).
    /// It's not efficient, use [Array::set] or [Dictionary::insert] when
    /// possible.
    ///
    /// # Panics
    /// This function panics if the `new_value` is either an Array, Dictionary,
    /// Key or Null. They are not supported by the `libplist` in this scenario.
    /// Use [Array::set] for arrays or [Dictionary::insert] for dictionaries to change their values.
    pub fn replace_with(&mut self, new_value: &Value) {
        let pointer = self.as_node().pointer();
        let false_drop = self.as_node().false_drop();
        let mut new_self = match new_value {
            Value::Boolean(boolean) => unsafe {
                unsafe_bindings::plist_set_bool_val(pointer, boolean.as_bool().into());
                from_pointer(pointer)
            },
            Value::Data(data) => unsafe {
                unsafe_bindings::plist_set_data_val(
                    pointer,
                    data.as_bytes().as_ptr() as *const _,
                    data.len(),
                );
                from_pointer(pointer)
            },
            Value::Date(date) => unsafe {
                let mut sec = std::mem::zeroed();
                let mut usec = std::mem::zeroed();
                unsafe_bindings::plist_get_date_val(date.pointer, &mut sec, &mut usec);
                unsafe_bindings::plist_set_date_val(pointer, sec, usec);
                from_pointer(pointer)
            },
            Value::Integer(integer) => unsafe {
                unsafe_bindings::plist_set_uint_val(pointer, integer.as_unsinged());
                from_pointer(pointer)
            },
            Value::Real(real) => unsafe {
                unsafe_bindings::plist_set_real_val(pointer, real.as_float());
                from_pointer(pointer)
            },
            Value::PString(string) => unsafe {
                let ptr =
                    unsafe_bindings::plist_get_string_ptr(string.pointer, std::ptr::null_mut());
                unsafe_bindings::plist_set_string_val(pointer, ptr);
                from_pointer(pointer)
            },
            Value::Uid(uid) => unsafe {
                unsafe_bindings::plist_set_uid_val(pointer, uid.get());
                from_pointer(pointer)
            },
            Value::Array(_) | Value::Dictionary(_) | Value::Key(_) | Value::Null(_) => {
                panic!("Replacing a plist node with a such value is not supported")
            }
        };
        // The old plist shoudn't be dropped, the pointer remains the same
        self.as_node_mut().set_false_drop(true);
        new_self.as_node_mut().set_false_drop(false_drop);
        *self = new_self;
    }
}

impl TryFrom<Value<'_>> for Vec<u8> {
    type Error = Error;

    fn try_from(value: Value<'_>) -> Result<Self, Self::Error> {
        value.to_bytes()
    }
}

// I couldn't implement the standart Clone trait because of lifetimes.
// Cloned value must have a lifetime of a function caller, not
// of the old value.
impl Value<'_> {
    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'a>(&self) -> Value<'a> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.as_node().pointer()) };
        unsafe { from_pointer(pointer) }
    }
}

/*
impl Clone for Value<'_> {
    fn clone(&self) -> Self {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.as_node().pointer()) };
        unsafe { from_pointer(pointer) }
    }
}

fn it_fails() {
    let a = Array::new();
    let b = a.get(0).unwrap().clone();
    std::mem::drop(a); // doesn't compile
    b.as_array();
}
 */

/// Creates a new plist value from the a C pointer. A pointer should be created
/// using the `libplist` library.
///
/// # Safety
/// Use this function only when dealing with other C libraries / Rust FFI wrappers which
/// use `libplist`. Passing an incorrent pointer will cause undefined behavior.
///
/// # Panics
/// May panic if an incorrect pointer has been passed and it was recognized on the C side.
pub unsafe fn from_pointer<'a>(pointer: unsafe_bindings::plist_t) -> Value<'a> {
    let typ: NodeType = unsafe { unsafe_bindings::plist_get_node_type(pointer) }.into();
    match typ {
        NodeType::Array => Value::Array(Array {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Boolean => Value::Boolean(Boolean {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Data => Value::Data(Data {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Date => Value::Date(Date {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Dictionary => Value::Dictionary(Dictionary {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Integer => Value::Integer(Integer {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Key => Value::Key(Key {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Null => Value::Null(Null {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Real => Value::Real(Real {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::String => Value::PString(PString {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
        NodeType::Uid => Value::Uid(Uid {
            pointer,
            false_drop: false,
            phantom: std::marker::PhantomData,
        }),
    }
}

/// Parses a JSON string and returns a [Value] struct representing a plist.
pub fn from_json<'a>(json: impl Into<String>) -> Result<Value<'a>, Error> {
    let json = CString::new(json.into())?;
    let json_len: u32 = json.as_bytes().len() as u32;
    let mut plist_t = unsafe { std::mem::zeroed() };
    let result = unsafe { unsafe_bindings::plist_from_json(json.as_ptr(), json_len, &mut plist_t) };
    if result != PLIST_ERROR_SUCCESS {
        return Err(result.into());
    }
    Ok(unsafe { from_pointer(plist_t) })
}

/// Parses an XML string and returns a [Value] struct representing a plist.
pub fn from_xml<'a>(xml: impl Into<String>) -> Result<Value<'a>, Error> {
    let xml = CString::new(xml.into())?;
    let xml_len: u32 = xml.as_bytes().len() as u32;
    let mut plist_t = unsafe { std::mem::zeroed() };
    let result = unsafe { unsafe_bindings::plist_from_xml(xml.as_ptr(), xml_len, &mut plist_t) };
    if result != PLIST_ERROR_SUCCESS {
        return Err(result.into());
    }
    Ok(unsafe { from_pointer(plist_t) })
}

/// Parses a slice of bytes as a binary plist and returns a [Value] struct.
pub fn from_binary<'a>(bytes: &[u8]) -> Result<Value<'a>, Error> {
    let mut plist_t = unsafe { std::mem::zeroed() };
    let result = unsafe {
        unsafe_bindings::plist_from_bin(bytes.as_ptr() as *mut _, bytes.len() as u32, &mut plist_t)
    };
    if result != PLIST_ERROR_SUCCESS {
        return Err(result.into());
    }
    Ok(unsafe { from_pointer(plist_t) })
}

/// Parses OpenStep ASCII string and returns a [Value] struct representing a plist.
pub fn from_openstep<'a>(xml: impl Into<String>) -> Result<Value<'a>, Error> {
    let openstep = CString::new(xml.into())?;
    let openstep_len: u32 = openstep.as_bytes().len() as u32;
    let mut plist_t = unsafe { std::mem::zeroed() };
    let result = unsafe {
        unsafe_bindings::plist_from_openstep(
            openstep.as_ptr() as *const _,
            openstep_len,
            &mut plist_t,
        )
    };
    if result != PLIST_ERROR_SUCCESS {
        return Err(result.into());
    }
    Ok(unsafe { from_pointer(plist_t) })
}

/// Parses a slice of bytes, determines its plist format and returns a [Value] struct representing a plist.
pub fn from_memory<'a>(bytes: &[u8]) -> Result<Value<'a>, Error> {
    let mut plist_t = unsafe { std::mem::zeroed() };
    let result = unsafe {
        unsafe_bindings::plist_from_memory(
            bytes.as_ptr() as *mut _,
            bytes.len() as u32,
            &mut plist_t,
            std::ptr::null_mut(),
        )
    };
    if result != PLIST_ERROR_SUCCESS {
        return Err(result.into());
    }
    Ok(unsafe { from_pointer(plist_t) })
}

/// Reads a file, determines its plist format and returns a [Value] struct representing a plist.
pub fn from_file<'a>(path: impl AsRef<std::path::Path>) -> Result<Value<'a>, Error> {
    let bytes = std::fs::read(path).map_err(|_| Error::IO)?;
    from_memory(&bytes)
}

mod plist_ffi {
    use crate::unsafe_bindings;

    /// A common trait for any plist node for dealing
    /// with underlying C structures.
    pub trait PlistFFI {
        /// Returns the pointer to a corresponding C structure.
        fn pointer(&self) -> unsafe_bindings::plist_t;

        /// Returns `true` if the plist is going to be *false* dropped.
        fn false_drop(&self) -> bool;

        /// Sometimes (eg. when returning a value of an array or dict) we
        /// need to *false* drop such values. Essentialy we create a pointer
        /// to a C struct and we don't own it. Freeing such values causes UB.
        fn set_false_drop(&mut self, value: bool);
    }
}
