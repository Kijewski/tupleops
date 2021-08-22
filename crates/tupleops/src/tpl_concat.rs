crate::do_impl!("concat", tuple_concat, {
    use crate::{prepend, Prepend, TuplePrepend};

    /// The resulting type when two tuples are concatenated.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::ConcatTuples;
    ///
    /// assert_same_types!(
    ///     ConcatTuples<(u8, u16), (u32, u64)>,
    ///     (u8, u16, u32, u64)
    /// );
    /// ```
    ///
    /// See also: [concat_tuples], [TupleConcat].
    #[cfg_attr(docsrs, doc(cfg(feature = "concat")))]
    pub type ConcatTuples<Front, Back> = <(Front, Back) as TupleConcat<Front, Back>>::Type;

    /// Concatenate two tuples.
    ///
    /// ```
    /// use tupleops::concat_tuples;
    ///
    /// assert_eq!(
    ///     concat_tuples((1, 2), (3, 4)),
    ///     (1, 2, 3, 4),
    /// );
    /// ```
    ///
    /// See also: [ConcatTuples], [TupleConcat].
    #[cfg_attr(docsrs, doc(cfg(feature = "concat")))]
    #[inline(always)]
    pub fn concat_tuples<Front, Back>(front: Front, back: Back) -> ConcatTuples<Front, Back>
    where
        (Front, Back): TupleConcat<Front, Back>,
    {
        <(Front, Back) as TupleConcat<Front, Back>>::concat_tuples(front, back)
    }

    /// Two tuples that are usable with [concat_tuples()].
    ///
    /// See also: [concat_tuples()], [TupleConcat].
    #[cfg_attr(docsrs, doc(cfg(feature = "concat")))]
    pub trait TupleConcat<Front, Back> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn concat_tuples(front: Front, back: Back) -> Self::Type;
    }

    impl<Back> TupleConcat<(), Back> for ((), Back) {
        type Type = Back;

        #[inline(always)]
        fn concat_tuples((): (), back: Back) -> Self::Type {
            back
        }
    }

    impl<Elem, Back> TupleConcat<(Elem,), Back> for ((Elem,), Back)
    where
        (Elem, Back): TuplePrepend<Elem, Back>,
    {
        type Type = Prepend<Elem, Back>;

        #[inline(always)]
        fn concat_tuples(front: (Elem,), back: Back) -> Self::Type {
            let (elem,) = front;
            prepend(elem, back)
        }
    }
});
