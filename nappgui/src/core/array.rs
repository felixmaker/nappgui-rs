use core::marker::PhantomData;
use std::{any::type_name, ffi::CString};

/// The Array (or Vector) is a container (or collection) of elements that are stored in contiguous memory locations.
/// This fact provides a series of advantages that make it the most used data structure and the one we should resort
/// to in the first instance.
pub struct NappguiArray<T> {
    pub(crate) inner: *mut nappgui_sys::Array,
    _marker: PhantomData<T>,
}

impl<T> NappguiArray<T> {
    /// Create an empty array.
    pub fn new() -> Self {
        let esize = size_of::<T>();
        let type_ = type_name::<T>();
        let type_ = CString::new(type_).unwrap();
        let inner = unsafe { nappgui_sys::array_create(esize as _, type_.as_ptr()) };
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Get the number of elements in an array.
    pub fn len(&self) -> usize {
        unsafe { nappgui_sys::array_size(self.inner) as _ }
    }

    /// Get the const item in pos position.
    pub fn get(&self, pos: usize) -> Option<&T> {
        let ptr = unsafe { nappgui_sys::array_get(self.inner, pos as _) };
        if !ptr.is_null() {
            unsafe { Some(&*(ptr as *mut T)) }
        } else {
            None
        }
    }

    /// Get the mutable item in pos position.
    pub fn get_mut(&mut self, pos: usize) -> Option<&mut T> {
        let ptr = unsafe { nappgui_sys::array_get(self.inner, pos as _) };
        if !ptr.is_null() {
            unsafe { Some(&mut *(ptr as *mut T)) }
        } else {
            None
        }
    }

    /// Insert an element at position `index` within the array
    pub fn insert(&mut self, index: usize, value: T) {
        let inserted = unsafe { nappgui_sys::array_insert(self.inner, index as _, 1) };
        unsafe {
            inserted.cast::<T>().write(value);
        }
    }

    /// Insert `n` elements at position `index` within the array.
    pub fn insert_n<I>(&mut self, index: usize, n: usize, array: I)
    where
        I: IntoIterator<Item = T>,
    {
        let inserted = unsafe { nappgui_sys::array_insert(self.inner, index as _, n as _) };

        for (i, item) in array.into_iter().enumerate() {
            if i == n {
                break;
            }
            let c_item = unsafe { inserted.cast::<T>().add(i) };
            unsafe { *c_item = item };
        }
    }

    /// Remove an element at position `index` within the array.
    pub fn remove(&mut self, index: usize) {
        self.remove_n(index, 1);
    }

    /// Remove `n` element at position `index` within the array.
    pub fn remove_n(&mut self, index: usize, n: usize) {
        unsafe extern "C" fn delete_func<T>(ptr: *mut libc::c_void) {
            let ptr = ptr as *mut T;
            drop(unsafe { ptr.read() });
        }
        unsafe {
            nappgui_sys::array_delete(self.inner, index as _, n as _, Some(delete_func::<T>));
        }
    }

    /// Delete the contents of the array, without destroying the container that will be left with zero elements.
    pub fn clear(&mut self) {
        unsafe extern "C" fn delete_func<T>(ptr: *mut libc::c_void) {
            let ptr = ptr as *mut T;
            drop(unsafe { ptr.read() });
        }
        unsafe {
            nappgui_sys::array_clear(self.inner, Some(delete_func::<T>));
        }
    }

    /// Get a cosnt pointer to the internal memory of the array, which gives direct access to all the elements.
    pub fn as_ptr(&self) -> *const T {
        unsafe { nappgui_sys::array_all(self.inner) as _ }
    }

    /// Get a pointer to the internal memory of the array, which gives direct access to all the elements.
    pub fn as_mut_ptr(&self) -> *mut T {
        unsafe { nappgui_sys::array_all(self.inner) as _ }
    }

    /// Get an iterator over the array.
    pub fn iter(&self) -> NappguiArrayIter<'_, T> {
        NappguiArrayIter::new(&self)
    }

    /// Creates an array from a vector.
    pub fn from_vec(&self, vec: Vec<T>) -> Self {
        let mut array = NappguiArray::new();
        array.insert_n(0, vec.len(), vec);
        array
    }

    /// Copies self into a new Vec.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::new();
        for item in self.iter() {
            vec.push(item.clone());
        }
        vec
    }
}

pub struct NappguiArrayIter<'a, T> {
    ptr: *const T,
    pos: usize,
    len: usize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> NappguiArrayIter<'a, T> {
    pub fn new(array: &'a NappguiArray<T>) -> Self {
        Self {
            ptr: array.as_ptr(),
            pos: 0,
            len: array.len(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for NappguiArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.len {
            let item = unsafe { &*(self.ptr.add(self.pos)) };
            self.pos = self.pos + 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_array_insert() {
        let mut array = NappguiArray::new();
        array.insert_n(0, 3, [1, 2, 3]);
        assert_eq!(array.get(1).unwrap().clone(), 2);

        if let Some(item) = array.get_mut(2) {
            *item = 4;
        }
        assert_eq!(array.get(2).unwrap().clone(), 4);

        array.insert(0, -1);
        assert_eq!(array.get(0).unwrap().clone(), -1);
    }

    #[test]
    fn test_array_remove() {
        let mut array = NappguiArray::new();
        array.insert_n(0, 3, [1, 2, 3]);
        array.remove(1);
        assert_eq!(array.get(1).unwrap().clone(), 3);
    }

    #[test]
    fn test_to_vec() {
        let mut array = NappguiArray::new();
        array.insert_n(0, 3, [1, 2, 3]);
        assert_eq!(array.len(), 3);
        let vec = array.to_vec();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
