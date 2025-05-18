use crate::{unsafe_bindings, Value};

crate::impl_node!(
    /// A boolean plist node.
    Boolean
);

impl Boolean<'_> {
    /// Creates a new boolean plist node.
    pub fn new(value: bool) -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_bool(value.into()) },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the value of the boolean.
    pub fn as_bool(&self) -> bool {
        let mut val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_bool_val(self.pointer, &mut val);
            !matches!(val, 0)
        }
    }

    /// Sets the value of the boolean.
    pub fn set(&mut self, value: bool) {
        unsafe { unsafe_bindings::plist_set_bool_val(self.pointer, value.into()) }
    }
}

impl From<bool> for Boolean<'_> {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<Boolean<'_>> for bool {
    fn from(value: Boolean<'_>) -> Self {
        value.as_bool()
    }
}

impl From<bool> for Value<'_> {
    fn from(value: bool) -> Self {
        Boolean::new(value).into()
    }
}

impl std::fmt::Display for Boolean<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_bool().fmt(f)
    }
}

impl Default for Boolean<'_> {
    fn default() -> Self {
        bool::default().into()
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Boolean<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_bool().fmt(f)
    }
}

impl PartialEq for Boolean<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bool() == other.as_bool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool() {
        let mut p = Boolean::new(false);
        assert_eq!(p.as_bool(), false);
        p.set(true);
        assert_eq!(p.as_bool(), true);
    }
}
