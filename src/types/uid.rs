use crate::unsafe_bindings;

crate::impl_node!(
    /// A plist `uid` plist node. These are found exclusively in plists created by NSKeyedArchiver.
    Uid
);

impl Uid<'_> {
    /// Creates a new uid plist node.
    pub fn new(uid: u64) -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_uid(uid) },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the value of the uid.
    pub fn get(&self) -> u64 {
        let mut uid = 0;
        unsafe {
            unsafe_bindings::plist_get_uid_val(self.pointer, &mut uid);
        }
        uid
    }

    /// Sets the uid with the given value.
    pub fn set(&mut self, uid: u64) {
        unsafe { unsafe_bindings::plist_set_uid_val(self.pointer, uid) }
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> Uid<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) })
            .into_uid()
            .unwrap()
    }
}

impl From<Uid<'_>> for u64 {
    fn from(value: Uid<'_>) -> Self {
        value.get()
    }
}

impl From<u64> for Uid<'_> {
    fn from(value: u64) -> Self {
        Uid::new(value)
    }
}

impl From<u32> for Uid<'_> {
    fn from(value: u32) -> Self {
        Uid::new(value as u64)
    }
}

impl From<u16> for Uid<'_> {
    fn from(value: u16) -> Self {
        Uid::new(value as u64)
    }
}

impl From<u8> for Uid<'_> {
    fn from(value: u8) -> Self {
        Uid::new(value as u64)
    }
}

impl PartialEq for Uid<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl Default for Uid<'_> {
    fn default() -> Self {
        u64::default().into()
    }
}

impl std::fmt::Display for Uid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Uid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}
