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

//! Functions to generate the code of [tupleop](https://crates.io/crates/tupleops) trait implementations.

mod common;
mod one_arg;

use std::fmt::Error;

use common::gen_range;

macro_rules! implement {
    () => {};

    ( $name:ident ) => {
        pub fn $name(dest: &mut String, from: usize, to: usize) -> Result<(), Error> {
            gen_range(dest, from, to, one_arg::$name)
        }
    };

    ( $name:ident $($names:ident)+ ) => {
        implement! { $name }
        implement! { $($names)+ }
    };
}

implement! {
    tuple_all_ok
    tuple_all_some
    tuple_append
    tuple_apply
    tuple_concat_many
    tuple_concat
    tuple_length
    tuple_map
    tuple_option
    tuple_prepend
    tuple_ref_mut
    tuple_ref
    tuple_tuple
    tuple_unappend
    tuple_unprepend
}
