use std::ffi::c_void;

/// ```
/// use cidre::objc;
///
/// let _pp = objc::AutoreleasePoolPage::push();
///
/// ```
#[repr(transparent)]
pub struct AutoreleasePoolPage(*const c_void);

impl AutoreleasePoolPage {
    pub fn push() -> AutoreleasePoolPage {
        unsafe { objc_autoreleasePoolPush() }
    }
}

impl Drop for AutoreleasePoolPage {
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.0) }
    }
}

#[link(name = "objc", kind = "dylib")]
extern "C" {
    fn objc_autoreleasePoolPush() -> AutoreleasePoolPage;
    fn objc_autoreleasePoolPop(ctx: *const c_void);
}
