use crate::{Value, unsafe_bindings};
use std::ffi::CString;

crate::impl_node!(
    /// A string plist node.
    PString
);

impl PString<'_> {
    /// Creates a new string plist node.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn new(string: impl Into<String>) -> Self {
        let string: CString = CString::new(string.into()).unwrap();
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_string(string.as_ptr() as *const _) },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the value of the string.
    pub fn as_str(&self) -> &'_ str {
        let mut len = 0;
        let ptr = unsafe { unsafe_bindings::plist_get_string_ptr(self.pointer, &mut len) };
        let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
        // TODO: add a check for correct utf-8 encoding?
        std::str::from_utf8(slice).unwrap()
    }

    /// Sets the value string with the given value.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn set(&mut self, string: impl Into<String>) {
        let c_string = CString::new(string.into()).unwrap();
        unsafe { unsafe_bindings::plist_set_string_val(self.pointer, c_string.as_ptr()) }
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> PString<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) })
            .into_string()
            .unwrap()
    }
}

impl From<String> for PString<'_> {
    fn from(value: String) -> Self {
        PString::new(value)
    }
}

impl From<&str> for PString<'_> {
    fn from(value: &str) -> Self {
        PString::new(value.to_string())
    }
}

impl From<PString<'_>> for String {
    fn from(value: PString<'_>) -> Self {
        String::from(value.as_str())
    }
}

impl From<&str> for Value<'_> {
    fn from(value: &str) -> Self {
        PString::new(value.to_string()).into()
    }
}

impl From<String> for Value<'_> {
    fn from(value: String) -> Self {
        PString::new(value.to_string()).into()
    }
}

impl PartialEq for PString<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl std::fmt::Display for PString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PString<'_> {
    fn default() -> Self {
        String::default().into()
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for PString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRING1: &str = "this is a string";
    const STRING2: &str = "this is a different string";

    #[test]
    fn string() {
        let mut p = PString::new(STRING1);
        assert_eq!(p.as_str(), STRING1);
        p.set(STRING2);
        assert_eq!(p.as_str(), STRING2);
    }
}
