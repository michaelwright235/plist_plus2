use crate::{unsafe_bindings, Value};
use core::ffi::c_char;

crate::impl_node!(
    /// A data plist node that is represented by a collection of bytes.
    Data
);

impl Data<'_> {
    /// Creates a new data plist node from a slice of bytes.
    pub fn new(data: &[u8]) -> Self {
        // plist_new_data copies the bytes so it's fine to pass a rust ptr
        Self {
            pointer: unsafe {
                unsafe_bindings::plist_new_data(data.as_ptr() as *const c_char, data.len() as u64)
            },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns a byte slice of the data value.
    pub fn as_bytes(&self) -> &'_ [u8] {
        let mut size = 0;
        let ptr = unsafe { unsafe_bindings::plist_get_data_ptr(self.pointer, &mut size) };
        unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) }
    }

    /// Returns an owned vector of the data value by copying it.
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    /// Returns the length of a contained bytes array.
    pub fn len(&self) -> u64 {
        self.as_bytes().len() as u64
    }

    /// Returns `true` if the data contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.as_bytes().len() == 0
    }

    /// Sets the contents to the given data.
    pub fn set(&mut self, bytes: &[u8]) {
        // The C function copies the bytes, it's fine to pass a pointer
        unsafe {
            unsafe_bindings::plist_set_data_val(
                self.pointer,
                bytes.as_ptr() as *mut c_char,
                bytes.len() as u64,
            )
        }
    }
}

impl From<Vec<u8>> for Data<'_> {
    fn from(bytes: Vec<u8>) -> Self {
        Data::new(&bytes)
    }
}

impl From<&[u8]> for Data<'_> {
    fn from(bytes: &[u8]) -> Self {
        Data::new(bytes)
    }
}

impl From<Vec<u8>> for Value<'_> {
    fn from(bytes: Vec<u8>) -> Self {
        Data::new(&bytes).into()
    }
}

impl From<&[u8]> for Value<'_> {
    fn from(bytes: &[u8]) -> Self {
        Data::new(bytes).into()
    }
}

impl From<Data<'_>> for Vec<u8> {
    fn from(data: Data<'_>) -> Self {
        data.to_vec()
    }
}

impl PartialEq for Data<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Default for Data<'_> {
    fn default() -> Self {
        Vec::default().into()
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Data<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_bytes().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: [u8; 5] = [1, 2, 3, 4, 5];
    const DATA2: [u8; 5] = [5, 4, 3, 2, 1];

    #[test]
    fn data() {
        let mut p = Data::new(&DATA1);
        assert_eq!(p.as_bytes(), DATA1);
        p.set(&DATA2);
        assert_eq!(p.as_bytes(), DATA2);
    }
}
