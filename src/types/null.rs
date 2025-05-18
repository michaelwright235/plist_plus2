use crate::unsafe_bindings;

crate::impl_node!(
    /// A null plist node.
    ///
    /// This type is not valid for all formats, e.g. the XML format does not support it.
    Null
);

impl Null<'_> {
    /// Creates a null plist node.
    pub fn new() -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_null() },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Null<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Null").finish()
    }
}

impl PartialEq for Null<'_> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Clone for Null<'_> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl Default for Null<'_> {
    fn default() -> Self {
        Self::new()
    }
}
