//! A `std` `Mutex` based implementation

use crate::*;
use std::sync::{Mutex, MutexGuard};

/// A wrap of `std::sync::Mutex`. This type's purpose is to allow choosing
/// between using `std::sync::Mutex` or `BlockingMutex` through feature flags.
///
/// # Example
///
/// ```
/// use mutex::RawMutex;
/// #[cfg(feature = "std")]
/// use mutex::StdBlockingMutex as Mutex;
/// #[cfg(not(feature = "std"))]
/// use mutex::BlockingMutex as Mutex;
///
/// struct MyData<R, T> {
///     mutex: Mutex<R, T>,
/// }
///
/// impl<R: RawMutex> MyData<R, u32> {
///    fn new(mutex: Mutex<R, u32>) -> Self {
///        Self { mutex }
///    }
/// }
///
/// #[cfg(feature = "std")]
/// fn test_select_mutex() {
///     use mutex::StdRawMutex;
///     let mutex = Mutex::<StdRawMutex, u32>::new(0);
///     let data = MyData::new(mutex);
/// }
///
/// #[cfg(not(feature = "std"))]
/// fn test_select_mutex() {
///     use crate::raw_impls::local::LocalRawMutex;
///     let mutex = Mutex::<LocalRawMutex, u32>::new(0);
///     let data = MyData::new(mutex);
/// }
/// ```
pub struct StdBlockingMutex<R, T> {
    mutex: Mutex<T>,
    _marker: PhantomData<R>,
}

impl<R, T> StdBlockingMutex<R, T> {
    /// Creates a new `StdBlockingMutex`.
    #[inline]
    pub const fn new(val: T) -> StdBlockingMutex<R, T> {
        Self {
            mutex: Mutex::new(val),
            _marker: PhantomData,
        }
    }

    /// lock
    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.mutex.lock().unwrap()
    }

    /// try_lock
    #[inline]
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.mutex.try_lock().ok()
    }

    /// with_lock
    #[inline]
    pub fn with_lock<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        let mut guard = self.lock();
        f(&mut *guard)
    }

    /// try_with_lock
    #[must_use]
    #[inline]
    pub fn try_with_lock<U>(&self, f: impl FnOnce(&mut T) -> U) -> Option<U> {
        let mut guard = self.try_lock()?;
        Some(f(&mut *guard))
    }
}

/// It only can be used with `StdBlockingMutex`.
/// It's not a real implementation.
pub struct StdRawMutex {}
unsafe impl RawMutex for StdRawMutex {
    type GuardMarker = *mut ();

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        false
    }

    unsafe fn unlock(&self) {}

    fn is_locked(&self) -> bool {
        true
    }
}
