use super::{Item, ItemMut};
use crate::{Value, Node, unsafe_bindings};
use core::ffi::c_void;

crate::impl_node!(
    /// An array plist node.
    Array
);

impl<'a> Array<'a> {
    /// Creates an empty array node.
    pub fn new() -> Self {
        Self {
            pointer: unsafe { unsafe_bindings::plist_new_array() },
            false_drop: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Returns the number of elements in the array.
    pub fn len(&self) -> u32 {
        unsafe { unsafe_bindings::plist_array_get_size(self.pointer) }
    }

    /// Returns `true` if the array contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn internal_get(&self, index: u32) -> Option<Value<'_>> {
        if index >= self.len() {
            return None;
        }
        let mut value = unsafe {
            crate::from_pointer(unsafe_bindings::plist_array_get_item(self.pointer, index))
        };
        value.as_node_mut().set_false_drop(true);
        Some(value)
    }

    /// Returns an immutable reference to the value corresponding to the index
    /// or [None] if out of bounds.
    pub fn get(&self, index: u32) -> Option<Item<'_>> {
        self.internal_get(index).map(Item)
    }

    /// Returns a mutable reference to the value corresponding to the index
    /// or [None] if out of bounds.
    pub fn get_mut(&mut self, index: u32) -> Option<ItemMut<'_>> {
        self.internal_get(index).map(ItemMut)
    }

    /// Sets the value of the index to the given value.
    ///
    /// The previous element of the same index is discarded.
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set<'b>(&mut self, value: impl Into<Value<'b>>, index: u32) {
        let len = self.len();
        if index >= len {
            panic!("inserting index (is {index}) should be < len (is {len})");
        }
        let mut value = value.into();
        value.as_node_mut().set_false_drop(true);
        unsafe {
            unsafe_bindings::plist_array_set_item(self.pointer, value.as_node().pointer(), index)
        };
    }

    /// Appends a new item at the end of the array.
    pub fn append<'b>(&mut self, value: impl Into<Value<'b>>) {
        let mut value = value.into();
        value.as_node_mut().set_false_drop(true);
        unsafe {
            unsafe_bindings::plist_array_append_item(self.pointer, value.as_node().pointer())
        };
    }

    /// Inserts an element at position index, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn insert<'b>(&mut self, value: impl Into<Value<'b>>, index: u32) {
        let len = self.len();
        if index >= len {
            panic!("inserting index (is {index}) should be < len (is {len})");
        }
        let mut value = value.into();
        value.as_node_mut().set_false_drop(true);
        unsafe {
            unsafe_bindings::plist_array_insert_item(self.pointer, value.as_node().pointer(), index)
        }
    }

    /// Removes an element at position index, shifting all elements after it to the left.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn remove(&mut self, index: u32) {
        let len = self.len();
        if index >= len {
            panic!("removal index (is {index}) should be < len (is {len})");
        }
        unsafe { unsafe_bindings::plist_array_remove_item(self.pointer, index) };
    }

    /// Creates an immutable iterator over the array.
    pub fn iter(&self) -> Iter<'_, 'a> {
        self.into_iter()
    }

    /// Creates a mutable iterator over the array.
    pub fn iter_mut(&mut self) -> IterMut<'_, 'a> {
        self.into_iter()
    }

    /// Returns a vector of [Values](Value) by copying array values.
    ///
    /// This operation requires copying every value into a new array.
    /// Since it's not efficient, try using [Array::iter] or [Array::iter_mut]
    /// when possible.
    pub fn to_vec<'b>(&self) -> Vec<Value<'b>> {
        // The values of an array should be cloned since it will be dropped
        // at the end.
        let mut values = Vec::with_capacity(self.len() as usize);
        for value in self {
            values.push(value.clone());
        }
        values
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones the value and gives it a lifetime of a caller.
    pub fn clone<'b>(&self) -> Array<'b> {
        let pointer = unsafe { unsafe_bindings::plist_copy(self.pointer) };
        (unsafe { crate::from_pointer(pointer) })
            .into_array()
            .unwrap()
    }
}

/// A helper macro for creating arrays.
///
/// # Example
/// ```rust
/// use plist_plus2::{array, Uid};
///
/// let arr = array!("Hello World", 123, Uid::new(4), -9.5);
/// println!("{arr:?}");
/// ```
#[macro_export]
macro_rules! array {
    () => {$crate::Array::new()};
    (
        $($val:expr),+
    ) => {
        {
            let mut array = $crate::Array::new();
            $(array.append($val);)+
            array
        }
    };
}

impl<'a> From<Vec<Value<'a>>> for Array<'_> {
    fn from(vec: Vec<Value<'a>>) -> Self {
        let mut array = Self::new();
        for item in vec {
            array.append(item);
        }
        array
    }
}

impl Default for Array<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Array<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (i, j) in self.iter().zip(other.iter()) {
            if i != j {
                return false;
            }
        }
        true
    }
}

#[cfg(feature = "clean_debug")]
impl std::fmt::Debug for Array<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = Vec::with_capacity(self.len() as usize);
        for item in self {
            values.push(item.0);
        }
        values.fmt(f)
    }
}

/// An immutable array iterator.
#[derive(Debug)]
pub struct Iter<'a, 'b> {
    iter_pointer: *mut c_void,
    array: &'a Array<'b>,
}

/// A mutable array iterator.
#[derive(Debug)]
pub struct IterMut<'a, 'b> {
    iter_pointer: *mut c_void,
    array: &'a mut Array<'b>,
}

impl<'a, 'b> IntoIterator for &'a Array<'b> {
    type Item = Item<'a>;
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

impl<'a, 'b> IntoIterator for &'a mut Array<'b> {
    type Item = ItemMut<'a>;
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

fn iter_next<'a>(array: &Array<'a>, iter_pointer: *mut c_void) -> Option<Value<'a>> {
    let mut to_fill = unsafe { std::mem::zeroed() };
    // Getting next item in array
    unsafe { unsafe_bindings::plist_array_next_item(array.pointer(), iter_pointer, &mut to_fill) };
    if to_fill.is_null() {
        // No more items in array
        None
    } else {
        // Getting type of next item in array
        let mut value = unsafe { crate::from_pointer(to_fill) };
        // False drop for iter and iter_mut
        value.as_node_mut().set_false_drop(true);
        Some(value)
    }
}

impl<'a> Iterator for Iter<'a, '_> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        iter_next(self.array, self.iter_pointer).map(Item)
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
    type Item = ItemMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        iter_next(self.array, self.iter_pointer).map(ItemMut)
    }
}

impl Drop for IterMut<'_, '_> {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.iter_pointer);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Array, Boolean, PString, Value};

    const ARRAY: [u64; 4] = [0, 1, 2, 3];

    #[test]
    fn array_append() {
        let boolean = Boolean::new(true);
        let mut array = Array::new();
        array.append(Value::Boolean(boolean));
        let result = array.get(0).unwrap().as_boolean().unwrap().as_bool();
        assert!(result);
    }

    #[test]
    fn array_get_item() {
        // Create a new array with 3 items
        let mut arr = Array::new();
        arr.append(PString::new("1"));
        arr.append(PString::new("2"));
        arr.append(PString::new("3"));

        // Get items and immediately drop them
        std::mem::drop(arr.get(0).unwrap());
        std::mem::drop(arr.get(1).unwrap());
        std::mem::drop(arr.get(2).unwrap());

        // Check if the items are still present.
        // They should be because we false drop them
        assert_eq!("1", arr.get(0).unwrap().as_string().unwrap().as_str());
        assert_eq!("2", arr.get(1).unwrap().as_string().unwrap().as_str());
        assert_eq!("3", arr.get(2).unwrap().as_string().unwrap().as_str());
    }

    #[test]
    fn array_to_vec() {
        // Create a new plist array [0, 1, 2, 3]
        let mut plist: Array<'_> = Array::new();
        for x in ARRAY {
            plist.append(Value::Integer(x.into()));
        }

        // Push the numbers into a new rust array.
        let mut numbers = Vec::with_capacity(ARRAY.len());
        for value in plist.to_vec() {
            numbers.push(value);
        }

        let mut iter = numbers.iter();

        // Check that numbers still exist and equal
        for x in ARRAY {
            assert_eq!(x, iter.next().unwrap().as_integer().unwrap().as_unsinged())
        }
    }

    #[test]
    fn array_iter() {
        // Create a new plist array [0, 1, 2, 3]
        let mut plist = Array::new();
        for x in ARRAY {
            plist.append(Value::Integer(x.into()));
        }

        // Ensure that the iterator created from a plist reference
        // will just false drop their items
        for item in &plist {
            std::mem::drop(item);
        }

        // Create a new interator once more and check the presence of
        // the values
        let mut iter = plist.iter();
        for x in ARRAY {
            assert_eq!(x, iter.next().unwrap().as_integer().unwrap().as_unsinged())
        }
    }

    #[test]
    fn array_iter_mut() {
        // Create a new plist array [9, 9, 9, 9]
        let mut plist = Array::new();
        for x in [9u64, 9, 9, 9] {
            plist.append(Value::Integer(x.into()));
        }
        println!("{plist:?}");

        // Set array items to [0, 1, 2, 3] with a mutable iter
        let mut mut_iter = plist.iter_mut();
        for x in ARRAY {
            mut_iter
                .next()
                .unwrap()
                .as_integer_mut()
                .unwrap()
                .set_unsigned(x);
        }
        std::mem::drop(mut_iter);

        // Check 'em
        let mut iter = plist.iter();
        for x in ARRAY {
            assert_eq!(x, iter.next().unwrap().as_integer().unwrap().as_unsinged())
        }
    }

    #[test]
    fn replace_with() {
        let mut a: Value = Boolean::new(true).into();
        let b: Value = PString::new("str").into();
        a.replace_with(&b);
        assert_eq!(a, b);
        std::mem::drop(b);
        assert_eq!(a.as_string().unwrap().as_str(), "str");
    }

    #[test]
    fn array_replace_with() {
        let mut a = array!(0);
        let b: Value<'_> = PString::new("world").into();
        a.get_mut(0).unwrap().replace_with(&b);
        assert_eq!(*a.get(0).unwrap(), b);
        std::mem::drop(b);
        assert_eq!(a.get(0).unwrap().as_string().unwrap().as_str(), "world");
    }
}
