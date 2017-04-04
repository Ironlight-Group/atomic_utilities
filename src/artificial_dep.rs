/// This module provides a function that forces an artificial data dependency
/// between two loads. Basically, the code:
///
/// ```text
/// val = some_atomic.load(DepOrd);
/// val2_ref = &val2;
/// val2 ref ^= val;
/// val2_ref ^= val; // val_2 ref now is equal to &val2, but data depends on val
/// loaded_val2 = *val2_ref; // Is ordered-after val as if by consume ordering
/// ```
/// is executed. This can be far faster than fences on arm and
/// power architectures, since the ordering is a result of data dependencies in
/// the pipeline and not full-on fences. This still isn't free, since you must
/// wait for the previous load to finish but it's better than a fence
///
///
/// # Example:
/// ```
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::sync::{Arc, Barrier};
/// use std::thread;
/// use atomic_utilities::artificial_dep::{DepOrd, dependently};
/// let num_run = 1000000;
/// let atomic_val1 = Arc::new(AtomicUsize::new(0));
/// let atomic_val2 = Arc::new(AtomicUsize::new(0));
/// let start_bar = Arc::new(Barrier::new(2));
/// let atomic_valt1 = atomic_val1.clone();
/// let atomic_valt2 = atomic_val2.clone();
/// let start_bart = start_bar.clone();
/// let to_join = thread::spawn(move || {
///     start_bart.wait();
///     for i in 0..num_run {
///         atomic_valt2.store(i, Ordering::Relaxed);
///         atomic_valt1.store(i, Ordering::Release);
///     }
/// });
///
/// start_bar.wait();
/// for _ in 0..num_run {
///     let val1_ld = atomic_val1.load(DepOrd);
///     let val2_ld = dependently(val1_ld, &atomic_val2,
///                              |dep_ref| dep_ref.load(Ordering::Relaxed));
///     assert!(val2_ld >= val1_ld); // Can fail if val2_ld is ordered_before val1_ld
/// }
/// ```
#[cfg(not(all(any(target_arch = "arm", target_arch = "aarch64"),
              use_asm)))]
mod artificial_dep_inner {
    use std::sync::atomic::Ordering;
    pub const DEPORD: Ordering = Ordering::Acquire;

    #[inline(always)]
    pub fn false_dep<T>(myref: &T, _v: usize) -> &T {
        myref
    }
}

#[cfg(all(any(target_arch = "arm", target_arch = "aarch64"),
          use_asm))]
mod artificial_dep_inner {
    use std::sync::atomic::Ordering;
    pub const DEPORD: Ordering = Ordering::Relaxed;

    #[inline(always)]
    pub fn false_dep<T>(mut myref: &T, val: usize) -> &T {
        unsafe {
            asm!("eor $0, $0, $1
              eor $0, $0, $1"
              : "+r" (myref)
              : "r" (val));
            myref
        }
    }
}

/* Once this can be tested on a power machine it's good to go
#[cfg(all(any(target_arch = "powerpc", target_arch = "powerpc64"),
          use_asm))]
mod artificial_dep_inner {
    use std::sync::atomic::Ordering;
    pub const DEPORD: Ordering = Ordering::Relaxed;

    #[inline(always)]
    pub fn false_dep<T>(myref: &T, val: usize) -> &T {
        asm!("xor $1 $0 $0
              xor $1 $0 $0"
              : "+r" (myref)
              : "r" (val));
        myref
    }
}*/

use std::sync::atomic::Ordering;

#[allow(non_upper_case_globals)]
pub const DepOrd: Ordering = artificial_dep_inner::DEPORD;

#[inline(always)]
pub fn dependently<T, R, F: FnOnce(&T) -> R>(val: usize, myref: &T, myfn: F) -> R {
    myfn(artificial_dep_inner::false_dep(myref, val))
}
