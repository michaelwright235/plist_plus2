use crate::{Node, unsafe_bindings};
use std::ffi::CString;

crate::impl_node!(
    /// A special plist node type representing a key of a dictionary.
    ///
    /// You can't create this type of node, only get it when dealing with
    /// a mutable dictionary iterator ([Dictionary::iter_mut](crate::dictionary::Dictionary::iter_mut)).
    /// Use it if you want to change the key of a value.
    Key
);

impl Key<'_> {
    /// Returns the key string of an associated dictionary value.
    pub fn get(&self) -> String {
        let mut key_ptr = std::ptr::null_mut();
        unsafe { unsafe_bindings::plist_get_key_val(self.pointer(), &mut key_ptr) };
        let key = unsafe {
            core::ffi::CStr::from_ptr(key_ptr)
                .to_string_lossy()
                .into_owned()
        };
        unsafe { unsafe_bindings::plist_mem_free(key_ptr as *mut _) };
        key
    }

    /// Sets the key of an associated dictionary value.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn set(&mut self, key: impl Into<String>) {
        let key = CString::new(key.into()).unwrap();
        // The C function makes a copy of a string, so we don't need to leak a CString
        unsafe { unsafe_bindings::plist_set_key_val(self.pointer(), key.as_ptr() as *const _) }
    }
}

impl From<Key<'_>> for String {
    fn from(value: Key<'_>) -> Self {
        value.get()
    }
}

impl PartialEq for Key<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl std::fmt::Display for Key<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Key<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
