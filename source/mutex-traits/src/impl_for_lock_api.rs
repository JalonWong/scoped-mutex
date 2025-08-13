use crate::{ConstInit, RawMutex};
use lock_api_0_4 as lock_api;

impl<R: lock_api::RawMutex> ConstInit for R {
    const INIT: Self = R::INIT;
}

unsafe impl<R: lock_api::RawMutex> RawMutex for R {
    type GuardMarker = R::GuardMarker;

    #[inline]
    #[track_caller]
    fn try_lock(&self) -> bool {
        lock_api::RawMutex::try_lock(self)
    }

    #[inline]
    #[track_caller]
    fn lock(&self) {
        lock_api::RawMutex::lock(self)
    }

    #[inline]
    #[track_caller]
    unsafe fn unlock(&self) {
        lock_api::RawMutex::unlock(self)
    }

    #[inline]
    #[track_caller]
    fn is_locked(&self) -> bool {
        lock_api::RawMutex::is_locked(self)
    }
}
