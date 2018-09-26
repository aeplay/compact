use std;
/// Specifies the 3 states that the pointer can be in:
/// 1. Free: On the heap - Stores a pointer
/// 2. Compact: On the dynamic part - Stores an offset
/// 3. Null
enum Inner {
    Free(u64),
    Compact(i32),
    Uninitialized,
}

/// See Inner
pub struct PointerToMaybeCompact<T> {
    inner: Inner,
    marker: ::std::marker::PhantomData<*mut T>
}

impl<T> Default for PointerToMaybeCompact<T> {
    fn default() -> PointerToMaybeCompact<T> {
        PointerToMaybeCompact {
            inner: Inner::Uninitialized,
            marker: ::std::marker::PhantomData
        }
    }
}

impl<T> std::fmt::Debug for PointerToMaybeCompact<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ptr {:?}", self.to_string())
    }
}

impl<T> PointerToMaybeCompact<T> {
    /// Create a new pointer which is initialized to point on the heap
    pub fn new_free(ptr: *mut T) -> Self {
        PointerToMaybeCompact {
            inner: Inner::Free(ptr as u64),
            marker: ::std::marker::PhantomData
        }
    }

    /// Set the pointer to point on the heap
    pub fn set_to_free(&mut self, ptr: *mut T) {
        self.inner = Inner::Free(ptr as u64)
    }

    /// Set the pointer to point on the dynamic part of the data structure
    pub fn set_to_compact(&mut self, ptr: *mut T) {
        self.inner = Inner::Compact((ptr as isize - self as *const Self as isize) as i32);
    }

    /// Get a raw pointer to wherever it is pointing
    pub unsafe fn ptr(&self) -> *const T {
        match self.inner {
            Inner::Free(ptr) => ptr as *const T,
            Inner::Compact(offset) => (self as *const Self as *const u8).offset(offset as isize) as *const T,
            Inner::Uninitialized => ::std::ptr::null(),
        }
    }

    /// Get a mut pointer to wherever it is pointing
    pub unsafe fn mut_ptr(&mut self) -> *mut T {
        match self.inner {
            Inner::Free(ptr) => ptr as *mut T,
            Inner::Compact(offset) => (self as *mut Self as *mut u8).offset(offset as isize) as *mut T,
            Inner::Uninitialized => ::std::ptr::null_mut(),
        }
    }

    /// Check to see if pointer is on the dynamic part of the data structure
    pub fn is_compact(&self) -> bool {
        match self.inner {
            Inner::Free(_) => false,
            Inner::Compact(_) | Inner::Uninitialized => true,
        }
    }

    /// Deallocate a memory range starting at pointer if it is in free mode
    pub fn deallocate_if_free<A: ::simple_allocator_trait::Allocator>(&self, length: usize) {
        if let Inner::Free(ptr) = self.inner {
            unsafe {
                A::deallocate(ptr as *mut T, length);
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self.inner {
            Inner::Free(p) => format!("Free {:p}", p as *const T),
            Inner::Compact(i) => format!("Compact {:?}", i),
            Inner::Uninitialized => String::from("uninitialized"),
        }
    }
}