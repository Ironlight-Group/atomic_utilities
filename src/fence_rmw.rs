use std::sync::atomic::Ordering;

/// This mod provides orderings to use with RMW operations
/// that optimally handle the case when all loads and stores
/// after an RMW operation must be ordered after the operation.
/// # Example:
/// ```
/// use std::sync::atomic::{AtomicUsize, fence};
/// use fence_rmw::{RMWOrder, fence_rmw()};
/// let atomic_refcnt = AtomicUsize::new(0);
/// atomic_refcnt.fetch_add(1, RMWOrder);
/// 
/// // ... do work here
/// // This will be ordered after the store of the fetch_add
/// // and will use minimal fences for various hardware platforms
/// atomic_refcnt.fetch_sub(1, Ordering::Release);
/// ```

#[cfg(any(target_platform = "x86", target_platform = "x86_64"))]
mod internal_ordering {
    use std::sync::atomic::Ordering;
    pub const RMW_O: Ordering = Ordering::Acquire;

    #[inline(always)]
    pub fn the_fence() {}
}

#[cfg(not(any(target_platform = "x86", target_platform = "x86_64")))]
mod internal_ordering {
    use std::sync::atomic::{Ordering, fence};
    pub const RMW_O: Ordering = Ordering::Relaxed;
    pub fn the_fence() {fence(Ordering::SeqCst)}
}

#[allow(non_upper_case_globals)]
pub const RMWOrder: Ordering = internal_ordering::RMW_O;

#[inline(always)]
pub fn fence_rmw() { internal_ordering::the_fence() }