use std::fmt::Write;

use crate::common::{pattern_for, pattern_for2};

pub(crate) fn tuple_all_ok(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<{generics}> TupleAllOk<({results})> for ({results}) {{
    type Type = ({args});

    #[inline(always)]
    fn all_ok(tpl: ({results})) -> Result<Self::Type, ({results})> {{
        match tpl {{
            ({oks}) => Ok(({vals})),
            tpl => Err(tpl),
        }}
    }}
}}",
        generics = pattern_for2(to, "I", ", E", "")?,
        results = pattern_for2(to, "Result<I", ", E", ">")?,
        args = pattern_for(to, "I", "")?,
        oks = pattern_for(to, "Ok(i", ")")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_all_some(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<{args}> TupleAllSome<({options})> for ({options}) {{
    type Type = ({args});

    #[inline(always)]
    fn all_some(tpl: ({options})) -> Result<Self::Type, ({options})> {{
        match tpl {{
            ({somes}) => Ok(({vals})),
            tpl => Err(tpl),
        }}
    }}
}}",
        args = pattern_for(to, "I", "")?,
        options = pattern_for(to, "Option<I", ">")?,
        somes = pattern_for(to, "Some(i", ")")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_append(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<Last, {args}> TupleAppend<({args}), Last> for (({args}), Last) {{
    type Type = ({args} Last);

    #[inline(always)]
    fn append(init: ({args}), last: Last) -> Self::Type {{
        let ({vals}) = init;
        ({vals} last)
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_apply(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<F, R, {args}> TupleApply<&F, ({args})> for (&F, ({args}))
where
    F: Fn({args}) -> R,
{{
    type Type = R;

    #[inline(always)]
    fn apply(func: &F, tpl: ({args})) -> Self::Type {{
        let ({vals}) = tpl;
        func({vals})
    }}
}}

impl<F, R, {args}> TupleApply<&mut F, ({args})> for (&mut F, ({args}))
where
    F: FnMut({args}) -> R,
{{
    type Type = R;

    #[inline(always)]
    fn apply(func: &mut F, tpl: ({args})) -> Self::Type {{
        let ({vals}) = tpl;
        func({vals})
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_concat_many(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<{args} Z1, Z2> TupleConcatMany<({args} Z1, Z2)> for ({args} Z1, Z2)
where
    (Z1, Z2): TupleConcat<Z1, Z2>,
    ({args} ConcatTuples<Z1, Z2>): TupleConcatMany<({args} ConcatTuples<Z1, Z2>)>
{{
    type Type = ConcatMany<({args} ConcatTuples<Z1, Z2>)>;

    #[inline(always)]
    fn concat_many(tpls: ({args} Z1, Z2)) -> Self::Type {{
        let ({vals} z1, z2) = tpls;
        let z = concat_tuples(z1, z2);
        concat_many(({vals} z))
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_concat(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<Back, Elem, {args}> TupleConcat<({args} Elem), Back> for (({args} Elem), Back)
where
    (Elem, Back): TuplePrepend<Elem, Back>,
    (({args}), Prepend<Elem, Back>): TupleConcat<({args}), Prepend<Elem, Back>>,
{{
    type Type = ConcatTuples<({args}), Prepend<Elem, Back>>;

    #[inline(always)]
    fn concat_tuples(front: ({args} Elem), back: Back) -> Self::Type {{
        let ({vals} elem) = front;
        concat_tuples(({vals}), prepend(elem, back))
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_length(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<{args}> TupleLength for ({args}) {{
    const LENGTH: usize = {to};
}}",
        args = pattern_for(to, "I", "")?,
        to = to,
    )
}

pub(crate) fn tuple_map(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<Mapper: TupleMapper, {args}> TupleMap<Mapper, ({args})> for (Mapper, ({args})) {{
    type Type = ({mapped_args});

    fn map_tuple(mut mapper: Mapper, tpl: ({args})) -> Self::Type {{
        let ({vals}) = tpl;
        ({mapped_vals})
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
        mapped_args = pattern_for(to, "<Mapper as TupleMapper>::MapElem::<I", ">")?,
        mapped_vals = pattern_for(to, "<Mapper as TupleMapper>::map_elem(&mut mapper, i", ")")?,
    )
}

pub(crate) fn tuple_option(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    let nones = "None, ".repeat(to);
    write!(
        dest,
        "\
impl<{args}> TupleOption<({args})> for ({args}) {{
    type Type = ({mapped_args});

    fn option_tuple(tpl: Option<({args})>) -> Self::Type {{
        match tpl {{
            Some(({vals})) => ({mapped_vals}),
            None => ({nones}),
        }}
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
        mapped_args = pattern_for(to, "Option<I", ">")?,
        mapped_vals = pattern_for(to, "Some(i", ")")?,
        nones = nones.trim_end(),
    )
}

pub(crate) fn tuple_prepend(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<Head, {args}> TuplePrepend<Head, ({args})> for (Head, ({args})) {{
    type Type = (Head, {args});

    #[inline(always)]
    fn prepend(head: Head, tail: ({args})) -> Self::Type {{
        let ({vals}) = tail;
        (head, {vals})
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_ref_mut(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<'a, {args}> TupleRefMut<'a, ({args})> for &'a mut ({args}) {{
    type Type = ({ref_args});

    #[inline(always)]
    fn ref_mut_tuple(tpl: &'a mut ({args})) -> Self::Type {{
        let ({vals}) = tpl;
        ({vals})
    }}
}}",
        ref_args = pattern_for(to, "&'a mut I", "")?,
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_ref(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<'a, {args}> TupleRef<'a, ({args})> for &'a ({args}) {{
    type Type = ({ref_args});

    #[inline(always)]
    fn ref_tuple(tpl: &'a ({args})) -> Self::Type {{
        let ({vals}) = tpl;
        ({vals})
    }}
}}",
        ref_args = pattern_for(to, "&'a I", "")?,
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_tuple(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<{args}> Tuple for ({args})
{{}}",
        args = pattern_for(to, "I", "")?,
    )
}

pub(crate) fn tuple_unappend(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<Elem, {args}> TupleUnappend<({args} Elem)> for ({args} Elem) {{
    type Init = ({args});
    type Last = Elem;

    #[inline(always)]
    fn unappend(tpl: ({args} Elem)) -> (Self::Init, Self::Last) {{
        let ({vals} last) = tpl;
        (({vals}), last)
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}

pub(crate) fn tuple_unprepend(dest: &mut String, to: usize) -> Result<(), std::fmt::Error> {
    write!(
        dest,
        "\
impl<Elem, {args}> TupleUnprepend<(Elem, {args})> for (Elem, {args}) {{
    type Head = Elem;
    type Tail = ({args});

    #[inline(always)]
    fn unprepend(tpl: (Elem, {args})) -> (Self::Head, Self::Tail) {{
        let (head, {vals}) = tpl;
        (head, ({vals}))
    }}
}}",
        args = pattern_for(to, "I", "")?,
        vals = pattern_for(to, "i", "")?,
    )
}
