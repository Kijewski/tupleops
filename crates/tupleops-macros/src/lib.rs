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

//! Procedural macros needed to implement [tupleop](https://crates.io/crates/tupleops).

mod common;

use proc_macro::TokenStream;

use common::generate;

macro_rules! implement {
    () => {};

    ( $name:ident ) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            generate(input, tupleops_generator::$name)
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
