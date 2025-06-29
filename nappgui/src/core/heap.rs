use std::{
    ffi::CString,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    rc::Rc,
};

/// Heap is a very efficient dynamic memory manager and auditor included in the core library and available for all
/// NAppGUI-based projects (libraries and applications). It is common for applications to request a large number
/// of small blocks of memory to hold different objects (character strings, interface controls, structure instances,
/// I/O buffers, etc.). The strategy behind the manager is none other than asking the operating system for memory
/// pages of a certain size (64kb or more) using bmem_malloc and using them to resolve multiple requests more
/// efficiently.
pub struct NappguiHeap<T> {
    pub(crate) inner: Rc<*mut u8>,
    _marker: PhantomData<T>,
}

impl<T> NappguiHeap<T> {
    /// Reserve memory for an object.
    pub fn new(value: T) -> Self {
        let value = &value as *const T as *const u8;

        let inner = unsafe {
            let size = std::mem::size_of::<T>();
            let name = std::any::type_name::<T>();
            let name = CString::new(name).unwrap();
            nappgui_sys::heap_malloc_imp(size as _, name.as_ptr(), 1)
        };

        unsafe {
            nappgui_sys::bmem_move(inner, value, 1);
        }

        NappguiHeap {
            inner: Rc::new(inner),
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for NappguiHeap<T> {
    fn drop(&mut self) {
        if Rc::strong_count(&self.inner) == 1 {
            unsafe {
                let size = std::mem::size_of::<T>();
                let name = std::any::type_name::<T>();
                let name = CString::new(name).unwrap();
                let mut ptr = *self.inner;
                nappgui_sys::heap_free(&mut ptr, size as _, name.as_ptr());
            }
        }
    }
}

impl<T> Deref for NappguiHeap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = *self.inner as *const T;
        unsafe { &*ptr }
    }
}

impl<T> DerefMut for NappguiHeap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = *self.inner as *mut T;
        unsafe { &mut *ptr }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_heap_new() {
        let mut heap = NappguiHeap::new(0);
        *heap = 5;
        assert_eq!(*heap, 5)
    }
}
