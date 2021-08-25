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
#![warn(missing_docs)]
#![no_std]
#![cfg_attr(
    feature = "feature-const_fn_trait_bound",
    feature(const_fn_trait_bound)
)]
#![cfg_attr(
    feature = "feature-generic_associated_types",
    feature(generic_associated_types)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Utility library to work with tuples.
//!
//! ## Features:
//!
//! * **Test if all elements are [Ok]: [all_ok()]**
//!
//!   `features = ["all-ok"]`, included by default
//!
//!   ```
//!   # use tupleops::all_ok;
//!   # fn good(value: i32) -> Result<i32, i32> {
//!   #     Ok(value)
//!   # }
//!   # fn bad(value: i32) -> Result<i32, i32> {
//!   #     Err(value)
//!   # }
//!   assert_eq!(
//!       all_ok((good(1), good(2), good(3))),
//!       Ok((1, 2, 3)),
//!   );
//!   assert_eq!(
//!       all_ok((good(1), bad(2), good(3))),
//!       Err((Ok(1), Err(2), Ok(3)))
//!   );
//!   ```
//!
//! * **Test if all elements are [Some]: [all_some()]**
//!
//!   `features = ["all-some"]`, included by default
//!
//!   ```
//!   # use tupleops::all_some;
//!   assert_eq!(
//!       all_some((Some(1), Some(2), Some(3))),
//!       Ok((1, 2, 3))
//!   );
//!   assert_eq!(
//!       all_some((Some(1), Option::<()>::None, Some(3))),
//!       Err((Some(1), None, Some(3)))
//!   );
//!   ```
//!
//! * **Prepend an element to a tuple: [prepend()]**
//!
//!   `features = ["prepend"]`, included by default
//!
//!   ```
//!   # use tupleops::prepend;
//!   assert_eq!(prepend(1, (2, 3, 4)), (1, 2, 3, 4));
//!   ```
//!
//! * **Append an element to a tuple: [append()]**
//!
//!   `features = ["append"]`, included by default
//!
//!   ```
//!   # use tupleops::append;
//!   assert_eq!(append((1, 2, 3), 4), (1, 2, 3, 4));
//!   ```
//!
//! * **Concatenate two tuples: [concat_tuples()]**
//!
//!   `features = ["concat"]`, included by default
//!
//!   ```
//!   # use tupleops::concat_tuples;
//!   assert_eq!(concat_tuples((1, 2), (3, 4, 5)), (1, 2, 3, 4, 5));
//!   ```
//!
//! * **Concatenate multiple tuples: [concat_many()]**
//!
//!   `features = ["concat-many"]`, included by default
//!
//!   ```
//!   # use tupleops::concat_many;
//!   assert_eq!(concat_many(((), (1,), (2, 3,), (4, 5, 6))), (1, 2, 3, 4, 5, 6));
//!   ```
//!
//! * **Turn a reference to a tuple to a tuple of references: [ref_tuple()]**
//!
//!   `features = ["ref"]`, included by default
//!
//!   ```
//!   # use tupleops::ref_tuple;
//!   assert_eq!(ref_tuple(&(1, 2, 3)), (&1, &2, &3));
//!   ```
//!
//! * **Turn a reference to a mutable tuple to a tuple of mutable references: [ref_mut_tuple()]**
//!
//!   `features = ["ref-mut"]`, included by default
//!
//!   ```
//!   # use tupleops::ref_mut_tuple;
//!   assert_eq!(ref_mut_tuple(&mut (1, 2, 3)), (&mut 1, &mut 2, &mut 3));
//!   ```
//!
//! * **Extract the first element of a tuple: [unprepend()]**
//!
//!   `features = ["unprepend"]`, included by default
//!
//!   ```
//!   # use tupleops::unprepend;
//!   assert_eq!(unprepend((1, 2, 3, 4)), (1, (2, 3, 4)));
//!   ```
//!
//! * **Extract the last element of a tuple: [unappend()]**
//!
//!   `features = ["unappend"]`, included by default
//!
//!   ```
//!   # use tupleops::unappend;
//!   assert_eq!(unappend((1, 2, 3, 4)), ((1, 2, 3), 4));
//!   ```
//!
//! * **Call a function with the tuple members as arguments: [apply()]**
//!
//!   `features = ["apply"]`, included by default
//!
//!   ```
//!   # use tupleops::apply;
//!   fn add3(a: u32, b: u32, c: u32) -> u32 { a + b + c }
//!
//!   let tpl3 = (1, 2, 3);
//!   assert_eq!(apply(&add3, tpl3), 6);
//!   ```
//!
//! * **Element-wise wrap the element of a tuple in [Option]: [option_tuple()]**
//!
//!   `features = ["option"]`, included by default
//!
//!   ```
//!   # use tupleops::option_tuple;
//!   assert_eq!(option_tuple(Some((1, 2, 3))), (Some(1), Some(2), Some(3)));
//!   ```
//!
//! * **Get the length of a tuple: [length()]**
//!
//!   `features = ["length"]`, included by default
//!
//!   ```
//!   # use tupleops::TupleLength;
//!   assert_eq!(<(u8, u16, u32) as TupleLength>::LENGTH, 3);
//!   ```
//!
//! * **Map a tuple: [map_tuple()]**
//!
//!   `features = ["[map](map_tuple)"]`, **not** included by default,
//!   because it needs the unstable feature
//!   [`generic_associated_types` (GAT)](https://github.com/rust-lang/rust/issues/44265).
//!
//!   ```
//!   # #![feature(generic_associated_types)]
//!   # use tupleops::{TupleMapper, map_tuple};
//!   struct MyTupleEnum(usize);
//!
//!   impl TupleMapper for MyTupleEnum {
//!       type MapElem<Type> = (usize, Type);
//!  
//!       fn map_elem<Elem>(&mut self, elem: Elem) -> Self::MapElem<Elem> {
//!           let index = self.0;
//!           self.0 += 1;
//!           (index, elem)
//!       }
//!   }
//!
//!   assert_eq!(
//!       map_tuple(MyTupleEnum(1), ("hello", "world", "!")),
//!       ((1, "hello"), (2, "world"), (3, "!")),
//!   )
//!   ```
//!
//! When used in libraries, you should probably use `default-features = false`, and only opt in
//! to the features you actually need.
//!
//! ## Supported tuple lengths:
//!
//! By default the selected operations are implemented to tuples upto a length of 16 elements
//! (`features = ["default-len"]`).
//! You can specify a higher limit by using `feature = ["X"]`, where X can be
//! 8, 16, 32, 64, 96, 128, 160, 192, 224, or 256. A higher number includes all lower numbers.
//!
//! **Beware:** `features = ["256"]` needs about 5 GB of RAM to compile the module,
//! so only use it if you actually need it.

mod tpl_all_ok;
mod tpl_all_some;
mod tpl_append;
mod tpl_apply;
mod tpl_concat;
mod tpl_concat_many;
mod tpl_length;
mod tpl_map;
mod tpl_option;
mod tpl_prepend;
mod tpl_ref;
mod tpl_ref_mut;
mod tpl_tuple;
mod tpl_unappend;
mod tpl_unprepend;
mod tpl_unprepend_template;

pub use tpl_all_ok::*;
pub use tpl_all_some::*;
pub use tpl_append::*;
pub use tpl_apply::*;
pub use tpl_concat::*;
pub use tpl_concat_many::*;
pub use tpl_length::*;
pub use tpl_map::*;
pub use tpl_option::*;
pub use tpl_prepend::*;
pub use tpl_ref::*;
pub use tpl_ref_mut::*;
pub use tpl_tuple::*;
pub use tpl_unappend::*;
pub use tpl_unprepend::*;
pub use tpl_unprepend_template::*;

#[doc(hidden)]
#[macro_export]
macro_rules! do_impl {
    ($feature_name:literal, $macro_name:ident, { $($body:tt)* }) => {
        pub use r#impl::*;

        #[cfg(not(feature = $feature_name))]
        mod r#impl {}

        #[cfg(feature = $feature_name)]
        mod r#impl {
            $($body)*

            ::tupleops_macros::$macro_name!(1..=4);

            #[cfg(feature = "8")]
            ::tupleops_macros::$macro_name!(5..=8);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "16")]
            ::tupleops_macros::$macro_name!(9..=16);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "32")]
            ::tupleops_macros::$macro_name!(17..=32);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "64")]
            ::tupleops_macros::$macro_name!(33..=64);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "96")]
            ::tupleops_macros::$macro_name!(65..=96);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "128")]
            ::tupleops_macros::$macro_name!(97..=128);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "160")]
            ::tupleops_macros::$macro_name!(129..=160);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "192")]
            ::tupleops_macros::$macro_name!(161..=192);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "224")]
            ::tupleops_macros::$macro_name!(193..=224);

            #[cfg(not(feature = "dont_hurt_yourself_by_using_all_features"))]
            #[cfg(feature = "256")]
            ::tupleops_macros::$macro_name!(225..=256);
        }
    };
}
