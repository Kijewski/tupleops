// Copyright (c) 2021 René Kijewski <rene.[SURNAME]@fu-berlin.de>
// All rights reserved.
//
// This software and the accompanying materials are made available under
// the terms of the ISC License which is available in the project root as LICENSE-ISC, AND/OR
// the terms of the MIT License which is available at in the project root as LICENSE-MIT, AND/OR
// the terms of the Apache License, Version 2.0 which is available in the project root as LICENSE-APACHE.
//
// You have to accept AT LEAST one of the aforementioned licenses to use, copy, modify, and/or distribute this software.
// At your will you may redistribute the software under the terms of only one, two, or all three of the aforementioned licenses.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

//! Ensure that two types are the same, or fail with a compilation error.
//!
//! ```
//! use same_types::assert_same_types;
//!
//! assert_same_types!(u32, u32, u32, u32);
//! ```
//!
//! ```compile_fail
//! use same_types::assert_same_types;
//!
//! // Fails with the message:
//! // the trait `SameTypes` is not implemented for `(i32, u32)`
//! assert_same_types!(u32, u32, i32, u32);
//! ```

use core::marker::PhantomData;

/// Helper trait for [assert_same_types] to tell if two types are exactly the same.
pub trait SameTypes {}

impl<T> SameTypes for (T, T) {}

/// Helper function for [assert_same_types] to tell if two types are exactly the same.
#[doc(hidden)]
#[inline(always)]
pub const fn same_types<A, B>(_: PhantomData<*const A>, _: PhantomData<*const B>)
where
    (A, B): SameTypes,
{
}

/// Assert that two or more types are exactly the same. Compilation error otherwise.
///
/// ```
/// use same_types::assert_same_types;
///
/// assert_same_types!(u32, u32, u32, u32);
/// ```
///
/// ```compile_fail
/// use same_types::assert_same_types;
///
/// assert_same_types!(u32, u32, i32, u32);
/// ```
#[macro_export]
macro_rules! assert_same_types {
    ($A:ty $(,)?) => {
    };
    ($A:ty, $B:ty $(,)?) => {
        $crate::same_types::<$A, $B>(::core::marker::PhantomData, ::core::marker::PhantomData);
    };
    ($A:ty, $B:ty, $($C:ty),+ $(,)?) => {
        $crate::assert_same_types!($A, $B);
        $crate::assert_same_types!($B, $($C),+);
    };
}
