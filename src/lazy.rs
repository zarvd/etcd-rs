#![allow(dead_code)]
use std::ops::{Deref, DerefMut};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A lock over lazily-instantiated data.
///
/// A `Lazy` holding a `T` is created from a closure returning a `T`.
/// This closure is not run until the `.read()` method is called.
pub struct Lazy<T> {
    /// The (locked, optional) contents of this Lazy. Initially `None`.
    inner: RwLock<Option<T>>,
    /// A thunk that will instantiate a `T`.
    factory: Box<dyn Fn() -> T + Sync + Send>,
}

impl<T> Lazy<T> {
    /// Create a new `Lazy` from a closure.
    ///
    /// This closure will be called lazily on the first request to access to the
    /// `Lazy`'s contents.
    pub fn new<F: 'static + Fn() -> T + Sync + Send>(factory: F) -> Self {
        Self {
            inner: Default::default(),
            factory: Box::new(factory),
        }
    }

    /// Acquire a read lock to the contents of this `Lazy`.
    ///
    /// If necessary, first instantiates these contents.
    pub async fn read(&self) -> LazyReadGuard<'_, T> {
        {
            let lock = self.inner.read().await;
            if lock.is_some() {
                return LazyReadGuard::new(lock);
            }
        }
        {
            let mut lock = self.inner.write().await;
            match *lock {
                None => {
                    let value = (self.factory)();
                    lock.replace(value);
                }
                Some(_) => {
                    // Someone beat us here! (They noticed that `self.inner` was
                    // `None` at about the same time.)
                    // That's fine; we'll use their value.
                }
            }
        }
        LazyReadGuard::new(self.inner.read().await)
    }

    pub async fn write(&self) -> LazyWriteGuard<'_, T> {
        let mut lock = self.inner.write().await;
        if lock.is_none() {
            let value = (self.factory)();
            lock.replace(value);
        }
        LazyWriteGuard::new(lock)
    }
}

/// The result of `Lazy<T>.read()`: holds a read lock over `T` and derefs to `T`.
///
/// Much like a `tokio::sync::RwLockReadGuard` (which this type wraps), a
/// `LazyReadGuard` will release a lock permit on `Drop`.
pub struct LazyReadGuard<'a, T> {
    inner: RwLockReadGuard<'a, Option<T>>,
}

impl<'a, T> LazyReadGuard<'a, T> {
    /// Create a new `LazyReadGuard<'_, T>` from the
    /// `tokio::sync::RwLockReadGuard<'_, Option<T>>` where the `Option<T> is
    /// known to be `Some`.
    fn new(inner: RwLockReadGuard<'a, Option<T>>) -> Self {
        assert!(
            inner.is_some(),
            "Should only instantiate LazyReadGuard with RwLockReadGuard over Some(_)."
        );
        LazyReadGuard { inner }
    }
}

impl<T> Deref for LazyReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
            .as_ref()
            .unwrap_or_else(|| unreachable!() /* see assert! in new() */)
    }
}

/// The result of `Lazy<T>.write()`: holds a write lock over `T` and derefs to `T`.
///
/// Much like a `tokio::sync::RwLockWriteGuard` (which this type wraps), a
/// `LazyWriteGuard` will release a lock permit on `Drop`.
pub struct LazyWriteGuard<'a, T> {
    inner: RwLockWriteGuard<'a, Option<T>>,
}

impl<'a, T> LazyWriteGuard<'a, T> {
    /// Create a new `LazyWriteGuard<'_, T>` from the
    /// `tokio::sync::RwLockWriteGuard<'_, Option<T>>` where the `Option<T> is
    /// known to be `Some`.
    fn new(inner: RwLockWriteGuard<'a, Option<T>>) -> Self {
        assert!(
            inner.is_some(),
            "Should only instantiate LazyWriteGuard with RwLockWriteGuard over Some(_)."
        );
        LazyWriteGuard { inner }
    }
}

impl<T> Deref for LazyWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
            .as_ref()
            .unwrap_or_else(|| unreachable!() /* see assert! in new() */)
    }
}

impl<T> DerefMut for LazyWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
            .as_mut()
            .unwrap_or_else(|| unreachable!() /* see assert! in new() */)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    const ORDER: Ordering = Ordering::SeqCst;

    /// Test getting a read lock from a `Lazy`.
    #[tokio::test]
    async fn test_lock_read() {
        let calls = Arc::new(AtomicUsize::default());

        let lazy = {
            let calls = calls.clone();
            Lazy::new(move || {
                calls.fetch_add(1, ORDER);
                true
            })
        };
        assert_eq!(calls.load(ORDER), 0, "Expected thunk not called.");

        let lock = lazy.read().await;
        assert_eq!((*lock), true, "Expected read() == thunk().");
        assert_eq!(calls.load(ORDER), 1, "Expected thunk called *once*.");

        // Should be able to acquire many read permits at once
        let lock2 = lazy.read().await;
        assert!(
            ptr::eq(&(*lock), &(*lock2)),
            "Expected read() to give *same reference*."
        );
        assert_eq!(calls.load(ORDER), 1, "Expected thunk *still* called once.");
    }

    /// Test getting a write lock from a `Lazy`.
    #[tokio::test]
    async fn test_lock_write() {
        let calls = Arc::new(AtomicUsize::default());

        let lazy = {
            let calls = calls.clone();
            Lazy::new(move || {
                calls.fetch_add(1, ORDER);
                true
            })
        };
        assert_eq!(calls.load(ORDER), 0, "Expected thunk not called.");

        {
            // need to let the write lock go out of scope before we can read
            let mut lock = lazy.write().await;
            assert_eq!((*lock), true, "Expected write() == thunk().");
            assert_eq!(calls.load(ORDER), 1, "Expected thunk called *once*.");

            *lock = false;
        }

        let lock = lazy.read().await;
        assert_eq!(calls.load(ORDER), 1, "Expected thunk *still* called once.");
        assert_eq!(*lock, false, "Expected read() to have been changed.");
    }
}
