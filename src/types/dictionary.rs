use crate::{plist_ffi::PlistFFI, unsafe_bindings, Value};
use core::ffi::c_void;
use std::ffi::CString;

use super::{Item, ItemMut, Key};

crate::impl_node!(
    /// A dictionary plist node.
    Dictionary
);

impl<'a> Dictionary<'a> {
    /// Creates an empty dictionary node.
    pub fn new() -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_dict() },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the number of elements in the dictionary.
    pub fn len(&self) -> u32 {
        unsafe { unsafe_bindings::plist_dict_get_size(self.pointer) }
    }

    /// Returns `true` if the dictionary contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn internal_get(&self, key: impl Into<String>) -> Option<Value<'_>> {
        let key_c_string = CString::new(key.into()).unwrap();
        let item_ptr =
            unsafe { unsafe_bindings::plist_dict_get_item(self.pointer, key_c_string.as_ptr()) };
        if item_ptr.is_null() {
            return None;
        }
        let mut item = unsafe { crate::from_pointer(item_ptr) };
        item.as_node_mut().set_false_drop(true);
        Some(item)
    }

    /// Returns an immutable reference to the value corresponding to the key
    /// or [None] if there's not a such key.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn get(&self, key: impl Into<String>) -> Option<Item<'_>> {
        self.internal_get(key).map(Item)
    }

    /// Returns an mutable reference to the value corresponding to the key
    /// or [None] if there's not a such key.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<ItemMut<'_>> {
        self.internal_get(key).map(ItemMut)
    }

    /// Inserts a key-value pair into the dictionary.
    ///
    /// If the dictionary did have this key present, the value is updated,
    /// and the old value is discarded.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn insert<'b>(&mut self, key: impl Into<String>, value: impl Into<Value<'b>>) {
        let key_c_string = CString::new(key.into()).unwrap();
        let mut value = value.into();
        value.as_node_mut().set_false_drop(true);
        unsafe {
            unsafe_bindings::plist_dict_set_item(
                self.pointer,
                key_c_string.as_ptr(),
                value.as_node().pointer(),
            )
        }
    }

    /// Removes a key from the dictionary.
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied string contains an internal 0 byte.
    pub fn remove(&mut self, key: impl Into<String>) {
        let key = key.into();
        if self.get(&key).is_none() {
            return;
        }
        let key = CString::new(key).unwrap();
        unsafe { unsafe_bindings::plist_dict_remove_item(self.pointer, key.as_ptr()) }
    }

    /// Merges a dictionary into another.
    ///
    /// This will copy all key/value pairs from the source dictionary to the current dictionary,
    /// overwriting any existing key/value pairs that are already present in target.
    pub fn merge(&mut self, from: &Dictionary) {
        // plist_dict_merge copies every node from another dictionary,
        // so passing a simple reference is fine
        unsafe { unsafe_bindings::plist_dict_merge(&mut self.pointer(), from.pointer()) }
    }

    /// Creates an immutable iterator over an dictionary.
    pub fn iter(&self) -> Iter<'_, 'a> {
        self.into_iter()
    }

    /// Creates a mutable iterator over an dictionary.
    pub fn iter_mut(&mut self) -> IterMut<'_, 'a> {
        self.into_iter()
    }

    /// Returns a tuple vector of keys and values by copying them.
    ///
    /// This operation requires copying every pair into a new array.
    /// Since it's not efficient, try using [Dictionary::iter] or [Dictionary::iter_mut]
    /// when possible.
    pub fn to_vec<'b>(&self) -> Vec<(String, Value<'b>)> {
        let mut v = Vec::with_capacity(self.len() as usize);
        for (key, item) in self.iter() {
            v.push((key, item.clone()));
        }
        v
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> Dictionary<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) }).into_dictionary().unwrap()
    }
}

impl Default for Dictionary<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Dictionary<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Returns `true` if `self` contains all of the same key-value pairs as `other`,
        // regardless of each dictionary's order
        if self.len() != other.len() {
            return false;
        }
        for (key, item) in self.iter() {
            if let Some(j) = other.get(key) {
                if item != j {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Dictionary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = Vec::with_capacity(self.len() as usize);
        for (key, item) in self {
            values.push((key, item));
        }
        f.debug_map().entries(values).finish()
    }
}

/// An immutable dictionary iterator.
#[derive(Debug)]
pub struct Iter<'a, 'b> {
    iter_pointer: *mut c_void,
    array: &'a Dictionary<'b>,
}

/// A mutable dictionary iterator.
#[derive(Debug)]
pub struct IterMut<'a, 'b> {
    iter_pointer: *mut c_void,
    array: &'a mut Dictionary<'b>,
}

impl<'a, 'b> IntoIterator for &'a Dictionary<'b> {
    type Item = (String, Item<'a>);
    type IntoIter = Iter<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter_pointer = unsafe { std::mem::zeroed() };
        unsafe { unsafe_bindings::plist_array_new_iter(self.pointer(), &mut iter_pointer) }
        Iter {
            iter_pointer,
            array: self,
        }
    }
}

impl<'a, 'b> IntoIterator for &'a mut Dictionary<'b> {
    type Item = (Key<'a>, ItemMut<'a>);
    type IntoIter = IterMut<'a, 'b>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter_pointer = unsafe { std::mem::zeroed() };
        unsafe { unsafe_bindings::plist_array_new_iter(self.pointer(), &mut iter_pointer) }
        IterMut {
            iter_pointer,
            array: self,
        }
    }
}

fn iter_next<'a>(dict: &Dictionary<'a>, iter_pointer: *mut c_void) -> Option<(Key<'a>, Value<'a>)> {
    let mut key_str = unsafe { std::mem::zeroed() };
    let mut value_ptr = unsafe { std::mem::zeroed() };
    // Getting next item in dictionary
    unsafe {
        unsafe_bindings::plist_dict_next_item(
            dict.pointer(),
            iter_pointer,
            &mut key_str,
            &mut value_ptr,
        )
    };
    if value_ptr.is_null() {
        // No more items in dictionary
        None
    } else {
        // Getting key of next item
        let key_ptr = unsafe { unsafe_bindings::plist_dict_item_get_key(value_ptr) };
        let mut key = unsafe { crate::from_pointer(key_ptr) };
        // Getting type of next item in dictionary
        let mut value = unsafe { crate::from_pointer(value_ptr) };
        // False drop for iter and iter_mut
        key.as_node_mut().set_false_drop(true);
        value.as_node_mut().set_false_drop(true);
        let key = key.into_key().unwrap(); // should be safe
        Some((key, value))
    }
}

impl<'a> Iterator for Iter<'a, '_> {
    type Item = (String, Item<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        iter_next(self.array, self.iter_pointer).map(|(k, v)| (k.get(), Item(v)))
    }
}

impl Drop for Iter<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.iter_pointer);
        }
    }
}

impl<'a> Iterator for IterMut<'a, '_> {
    type Item = (Key<'a>, ItemMut<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        iter_next(self.array, self.iter_pointer).map(|(k, v)| (k, ItemMut(v)))
    }
}

impl Drop for IterMut<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.iter_pointer);
        }
    }
}

impl<'a, K> From<Vec<(K, Value<'a>)>> for Dictionary<'_>
where
    K: Into<String>,
{
    fn from(value: Vec<(K, Value<'a>)>) -> Self {
        let mut dict = Self::new();
        for (key, value) in value {
            dict.insert(key, value);
        }
        dict
    }
}

/// A helper macro for creating dictionaries.
///
/// # Example
/// ```rust
/// use plist_plus2::dict;
///
/// let some_dict = dict!("hello" => "world", "some_it" => -34);
/// println!("{some_dict:?}");
/// ```
#[macro_export]
macro_rules! dict {
    () => {$crate::Dictionary::new()};
    (
        $($key:expr => $val:expr),+
    ) => {
        {
            let mut dict = $crate::Dictionary::new();
            $(dict.insert($key, $val);)+
            dict
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    const ARRAY: [u64; 4] = [0, 1, 2, 3];
    const KEYS: [&str; 4] = ["First", "Second", "Third", "Fourth"];

    #[test]
    fn dict_test() {
        let b = Boolean::new(false);
        let mut p = Dictionary::new();
        p.insert("b", b);
        let b = p.get("b").unwrap();
        assert_eq!(b.as_boolean().unwrap().as_bool(), false);
    }

    #[test]
    fn dict_to_vec() {
        // Create a new plist dict
        // ["First" => 1, "Second" => 2, "Third" => 3, "Fourth" => 4]
        let mut plist = Dictionary::new();
        for (key, value) in KEYS.into_iter().zip(ARRAY) {
            plist.insert(key, Integer::from(value));
        }

        // Push the values into a new array.
        let vec = plist.to_vec();
        let mut iter = vec.into_iter();

        // Check that values still exist and equal
        for (key, value) in KEYS.into_iter().zip(ARRAY) {
            let (plist_key, plist_value) = iter.next().unwrap();
            assert_eq!(key, plist_key);
            assert_eq!(value, plist_value.as_integer().unwrap().as_unsinged());
        }
    }

    #[test]
    fn dict_iter() {
        // Create a new plist dict
        // ["First" => 1, "Second" => 2, "Third" => 3, "Fourth" => 4]
        let mut plist = Dictionary::new();
        for (key, value) in KEYS.into_iter().zip(ARRAY) {
            plist.insert(key, Integer::from(value));
        }
        println!("{plist:?}");

        // Ensure that the iterator created from a plist reference
        // will just false drop their items
        for item in &plist {
            std::mem::drop(item);
        }

        // Create a new iterator once more and check the presence of
        // the values
        let mut iter = plist.iter();
        for (key, value) in KEYS.into_iter().zip(ARRAY) {
            let (i_key, i_value) = iter.next().unwrap();
            assert_eq!(key, i_key);
            assert_eq!(value, i_value.as_integer().unwrap().as_unsinged());
        }
    }

    #[test]
    fn dict_iter_mut() {
        // Create a new plist dict with dummy values
        let mut plist = Dictionary::new();
        for key in KEYS {
            plist.insert(key, PString::from("something"));
        }

        // Set dict items to the actual values
        // ["First" => 1, "Second" => 2, "Third" => 3, "Fourth" => 4]
        let mut mut_iter = plist.iter_mut();
        for x in ARRAY {
            mut_iter
                .next()
                .unwrap()
                .1
                .replace_with(&Integer::from(x).into());
        }
        std::mem::drop(mut_iter);

        // Check 'em
        let mut iter = plist.iter();
        for (key, value) in KEYS.into_iter().zip(ARRAY) {
            let (i_key, i_value) = iter.next().unwrap();
            assert_eq!(key, i_key);
            assert_eq!(value, i_value.as_integer().unwrap().as_unsinged());
        }
        std::mem::drop(iter);
        println!("{}", Value::Dictionary(plist).to_xml().unwrap());
    }
}
