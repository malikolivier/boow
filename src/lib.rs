//! # Borrowed-Or-oWned
//!
//! Provide a Borrowed-Or-oWned smart pointer.
//!
//! Alternative to [`Cow`] for which the [`Clone`] trait is not required for
//! the encapsulated type.
//!
//! Use this crate if you want something like [`Cow`] but your type cannot be
//! cloned.
//!
//! # How to use
//!
//! ```rust
//! extern crate boow;
//! use boow::Bow;
//!
//! // This struct contains a type for which we cannot know at compile time
//! // whether it will be owned or borrowed.
//! struct MyStruct<'a> {
//!     borrowed_or_owned: Bow<'a, InnerStruct>,
//! }
//!
//! struct InnerStruct {
//!     _stuff: String,
//! }
//!
//! impl<'a> MyStruct<'a> {
//!     // Use borrowed value
//!     fn from_borrowed(inner: &'a InnerStruct) -> Self {
//!         Self { borrowed_or_owned: Bow::Borrowed(inner) }
//!     }
//!
//!     // Use owned value
//!     fn from_owned(inner: InnerStruct) -> Self {
//!         Self { borrowed_or_owned: Bow::Owned(inner) }
//!     }
//! }
//! ```
//!
//! [`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::Borrow;
        use std::cmp::Ordering;
        use std::fmt;
        use std::hash::{Hash, Hasher};
        use std::ops::Deref;
    } else {
        extern crate alloc;
        use alloc::borrow::Borrow;
        use core::cmp::Ordering;
        use core::fmt;
        use core::hash::{Hash, Hasher};
        use core::ops::Deref;
    }
}

/// Borrow-Or-oWned smart pointer.
///
/// [`Bow`] implements [`Deref`], which means that you can call non-mutating
/// methods directly on the data it encloses. If mutation is desired,
/// [`borrow_mut`] will obtain some mutable reference to an owned value, but
/// only if it is owned.
///
/// [`borrow_mut`]: Bow::borrow_mut
#[derive(Copy, Clone)]
pub enum Bow<'a, T: 'a> {
    Owned(T),
    Borrowed(&'a T),
}

impl<'a, T: 'a> Borrow<T> for Bow<'a, T> {
    fn borrow(&self) -> &T {
        match *self {
            Bow::Owned(ref t) => t,
            Bow::Borrowed(t) => t,
        }
    }
}

impl<'a, T: 'a> Deref for Bow<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.borrow()
    }
}

impl<'a, T: 'a> Bow<'a, T> {
    /// Get a mutable reference to the enclosed value. Return [`None`] if the
    /// value is not owned.
    pub fn borrow_mut(&mut self) -> Option<&mut T> {
        match *self {
            Bow::Owned(ref mut t) => Some(t),
            Bow::Borrowed(_) => None,
        }
    }

    /// Consume the enclosed value and return it if it is owned.
    pub fn extract(self) -> Option<T> {
        match self {
            Bow::Owned(t) => Some(t),
            Bow::Borrowed(_) => None,
        }
    }
}

impl<'a, T: 'a> Eq for Bow<'a, T> where T: Eq {}

impl<'a, T: 'a> Ord for Bow<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Bow<'a, T>) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<'a, T: 'a> PartialEq for Bow<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Bow<'a, T>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
}

impl<'a, T: 'a> PartialOrd for Bow<'a, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Bow<'a, T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl<'a, T: 'a> fmt::Debug for Bow<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: 'a> fmt::Display for Bow<'a, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T: 'a> Default for Bow<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        Bow::Owned(T::default())
    }
}

impl<'a, T: 'a> Hash for Bow<'a, T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&**self, state)
    }
}

impl<'a, T: 'a> AsRef<T> for Bow<'a, T> {
    fn as_ref(&self) -> &T {
        self
    }
}
