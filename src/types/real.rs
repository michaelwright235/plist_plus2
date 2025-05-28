use crate::{Value, unsafe_bindings};

crate::impl_node!(
    /// A real `f64` plist node.
    Real
);

impl Real<'_> {
    /// Creates a new real plist node from a float.
    pub fn new(value: f64) -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_real(value) },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the value of the real.
    pub fn as_float(&self) -> f64 {
        let mut val = unsafe { std::mem::zeroed() };
        unsafe {
            unsafe_bindings::plist_get_real_val(self.pointer, &mut val);
        };
        val
    }

    /// Sets the value of the real with the given float.
    pub fn set(&mut self, value: f64) {
        unsafe { unsafe_bindings::plist_set_real_val(self.pointer, value) }
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> Real<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) })
            .into_real()
            .unwrap()
    }
}

impl From<f64> for Real<'_> {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<f64> for Value<'_> {
    fn from(value: f64) -> Self {
        Real::new(value).into()
    }
}

impl From<Real<'_>> for f64 {
    fn from(value: Real<'_>) -> Self {
        value.as_float()
    }
}

impl PartialEq for Real<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_float() == other.as_float()
    }
}

impl Default for Real<'_> {
    fn default() -> Self {
        f64::default().into()
    }
}

impl std::fmt::Display for Real<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_float().fmt(f)
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Real<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_float().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL1: f64 = 3.1415926;
    const REAL2: f64 = 1234.098765;

    #[test]
    fn real() {
        let mut p = Real::new(REAL1);
        assert_eq!(p.as_float(), REAL1);
        p.set(REAL2);
        assert_eq!(p.as_float(), REAL2);
    }
}
