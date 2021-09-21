use std::fmt;
use std::mem;
use std::os::raw::c_void;

/// C-ABI-compatible owned array.
/// Since each provider can use a different data structure or even a different allocator,
/// the struct needs to contain a destructor.
#[repr(C)]
pub struct PArray<T> {
    pub data: *mut T,
    pub len: usize,
    pub extra: *mut c_void,
    pub cleanup: unsafe extern "C" fn(*mut T, usize, *mut c_void),
}

impl<T> From<Vec<T>> for PArray<T> {
    fn from(mut v: Vec<T>) -> Self {
        v.shrink_to_fit();
        let ptr = v.as_mut_ptr();
        let len = v.len();
        let cap = v.capacity();

        unsafe extern "C" fn restore_vec<T>(ptr: *mut T, len: usize, extra: *mut c_void) {
            let cap = *Box::from_raw(extra as *mut usize);
            Vec::from_raw_parts(ptr, len, cap);
        }

        // Prevent running destructor
        mem::forget(v);

        PArray {
            data: ptr,
            len: len,
            extra: Box::into_raw(Box::new(cap)) as *mut c_void,
            cleanup: restore_vec,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for PArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len as isize {
            unsafe {
                write!(f, "{:?},", self.data.offset(i))?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Drop for PArray<T> {
    fn drop(&mut self) {
        unsafe { (self.cleanup)(self.data, self.len, self.extra) }
    }
}
