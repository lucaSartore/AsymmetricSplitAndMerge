use std::ffi::c_void;
use opencv::prelude::*;
use opencv::traits::Boxed;
use super::*;


/// represent a shallow copy of a Mat, that is a unsafe wrapper that simplify the coding removing
/// the lifetime from ImageContainerSplit
pub struct UnmanagedMat{
    pub image: Mat
}

// remind the user to call destroy so not to cause any double free crash
impl Drop for UnmanagedMat {
   fn drop(&mut self) {
        // swapping out the matrix with a new one so not to cause a panic
        let mut mat = Mat::default();
        std::mem::swap(&mut mat, &mut self.image);
        std::mem::forget(mat);
        panic!("Call drop on UnmanagedMat: unmanaged mat should never be tropped automatially since it may free the same object twice... try call the destroy method")
   } 
}

#[inline]
pub unsafe fn shallow_copy<T>(val: &T) -> T {
    let mut result = std::mem::MaybeUninit::uninit();
    std::ptr::copy_nonoverlapping(
        val as *const T as *const u8,
        result.as_mut_ptr() as *mut u8,
        std::mem::size_of::<T>()
    );
    result.assume_init()
}

impl UnmanagedMat {
    /// this function is unsafe as the caller must ensure that the borrowed ImageContainerSplit
    /// must stay in scope for as long as this UnmanagedMat is in scope
    pub unsafe fn from_image_container_split(img: &ImageContainerSplit<'_>) -> Self {
        let ptr = img.image.as_raw_Mat();
        let image = Mat::from_raw(ptr as *mut c_void);
        return Self{image}
    }

    pub fn destroy(self){
        let mat = unsafe{shallow_copy(&self.image)};
        std::mem::forget(mat);
        std::mem::forget(self);
    }
}
