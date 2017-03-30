
#[cfg(any(target_platform = "x86", target_platform = "x86_64",
          target_platform = "arm", target_platform = "aarch64",
          target_platform = "powerpc", target_platform = "powerpc64"))]
mod fences {
    #[inline(always)]
    pub fn load_conditional_store_fence() {}
}


#[cfg(not(any(target_platform = "x86", target_platform = "x86_64",
              target_platform = "arm", target_platform = "aarch64",
              target_platform = "powerpc", target_platform = "powerpc64")))]
mod fences {
    use std::sync::atomic::{Ordering, fence};
    #[inline(always)]
    pub fn load_conditional_store_fence() {
        fence(Ordering::Acquire);
    }
}

#[inline(always)]
pub fn load_conditional_store_fence() {
    fences::load_conditional_store_fence()
}