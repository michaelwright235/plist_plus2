use crate::{Value, unsafe_bindings};

crate::impl_node!(
    /// An integer that can be represented by either an `i64` or a `u64`.
    Integer
);

impl Integer<'_> {
    /// Creates a new integer plist node from a `u64`.
    pub fn new_unsigned(value: u64) -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_uint(value) },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Creates a new integer plist node from an `i64`.
    pub fn new_signed(value: i64) -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_int(value) },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the value of the integer as a `u64`.
    pub fn as_unsinged(&self) -> u64 {
        let mut val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_uint_val(self.pointer, &mut val);
        };
        val
    }

    /// Returns the value of the integer as an `i64`.
    pub fn as_singed(&self) -> i64 {
        let mut val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_int_val(self.pointer, &mut val);
        };
        val
    }

    /// Sets the integer value as a `u64`.
    pub fn set_unsigned(&mut self, value: u64) {
        unsafe { unsafe_bindings::plist_set_uint_val(self.pointer, value) }
    }

    /// Sets the integer value as an `i64`.
    pub fn set_signed(&mut self, value: i64) {
        unsafe { unsafe_bindings::plist_set_int_val(self.pointer, value) }
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> Integer<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) })
            .into_integer()
            .unwrap()
    }
}

impl From<Integer<'_>> for u64 {
    fn from(value: Integer<'_>) -> Self {
        value.as_unsinged()
    }
}

impl From<Integer<'_>> for i64 {
    fn from(value: Integer<'_>) -> Self {
        value.as_singed()
    }
}

impl From<u64> for Integer<'_> {
    fn from(value: u64) -> Self {
        Self::new_unsigned(value)
    }
}

impl From<u32> for Integer<'_> {
    fn from(value: u32) -> Self {
        Self::new_unsigned(value as u64)
    }
}

impl From<u16> for Integer<'_> {
    fn from(value: u16) -> Self {
        Self::new_unsigned(value as u64)
    }
}

impl From<u8> for Integer<'_> {
    fn from(value: u8) -> Self {
        Self::new_unsigned(value as u64)
    }
}

impl From<i64> for Integer<'_> {
    fn from(value: i64) -> Self {
        Self::new_signed(value)
    }
}

impl From<i32> for Integer<'_> {
    fn from(value: i32) -> Self {
        Self::new_signed(value as i64)
    }
}

impl From<i16> for Integer<'_> {
    fn from(value: i16) -> Self {
        Self::new_signed(value as i64)
    }
}

impl From<i8> for Integer<'_> {
    fn from(value: i8) -> Self {
        Self::new_signed(value as i64)
    }
}

impl From<u64> for Value<'_> {
    fn from(value: u64) -> Self {
        Integer::new_unsigned(value).into()
    }
}

impl From<u32> for Value<'_> {
    fn from(value: u32) -> Self {
        Integer::new_unsigned(value as u64).into()
    }
}

impl From<u16> for Value<'_> {
    fn from(value: u16) -> Self {
        Integer::new_unsigned(value as u64).into()
    }
}

impl From<u8> for Value<'_> {
    fn from(value: u8) -> Self {
        Integer::new_unsigned(value as u64).into()
    }
}

impl From<i64> for Value<'_> {
    fn from(value: i64) -> Self {
        Integer::new_signed(value).into()
    }
}

impl From<i32> for Value<'_> {
    fn from(value: i32) -> Self {
        Integer::new_signed(value as i64).into()
    }
}

impl From<i16> for Value<'_> {
    fn from(value: i16) -> Self {
        Integer::new_signed(value as i64).into()
    }
}

impl From<i8> for Value<'_> {
    fn from(value: i8) -> Self {
        Integer::new_signed(value as i64).into()
    }
}

impl PartialEq for Integer<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Note: since the plist numbers can be i64 or u64,
        // collisions may occur. For instance, -1 (i64) and
        // 18446744073709551615 (u64) are equal.
        self.as_unsinged() == other.as_unsinged()
    }
}

impl Default for Integer<'_> {
    fn default() -> Self {
        u64::default().into()
    }
}

// Probably we need to print both unsinged and singed representations
// of the integer?
impl std::fmt::Display for Integer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_singed().fmt(f)
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Integer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_singed().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UINT1: u64 = 123412340987;
    const UINT2: i64 = -98709781234;

    #[test]
    fn int() {
        let mut p = Integer::new_unsigned(UINT1);
        assert_eq!(p.as_unsinged(), UINT1);
        p.set_signed(UINT2);
        assert_eq!(p.as_singed(), UINT2);
    }
}
