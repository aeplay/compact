use super::Compact;
use compact_vec::CompactVec;

#[derive(Clone)]
pub struct Opaque {
    data: CompactVec<u8>
}

impl Opaque {
    pub fn new<T: Compact>(specific: T) -> Self {
        let data = CompactVec::with_capacity(specific.total_size_bytes());
        unsafe {
            Compact::compact_behind(&mut specific as *mut T, data.as_mut_ptr() as *mut T);
        }
        ::std::mem::forget(specific);
        Opaque {
            data
        }
    }
}

impl Compact for Opaque {
    fn is_still_compact(&self) -> bool {
        self.data.is_still_compact()
    }

    fn dynamic_size_bytes(&self) -> usize {
        self.data.dynamic_size_bytes()
    }

    unsafe fn compact(source: *mut Self, dest: *mut Self, new_dynamic_part: *mut u8) {
        Compact::compact(&mut (*source).data, &mut (*dest).data, new_dynamic_part)
    }

    unsafe fn decompact(source: *const Self) -> Self {
        Opaque {
            data: Compact::decompact(&(*source).data),
        }
    }
}