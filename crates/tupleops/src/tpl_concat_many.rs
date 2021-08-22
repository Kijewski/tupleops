crate::do_impl!("concat-many", tuple_concat_many, {
    use crate::{concat_tuples, ConcatTuples, TupleConcat};

    /// The resulting type when two or more tuples are concatenated.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::ConcatMany;
    ///
    /// assert_same_types!(
    ///     ConcatMany<()>,
    ///     ()
    /// );
    ///
    /// assert_same_types!(
    ///     ConcatMany<((u8, u16),)>,
    ///     (u8, u16)
    /// );
    ///
    /// assert_same_types!(
    ///     ConcatMany<((u8, u16), (u32, u64), (i8, i16), (i32, i64))>,
    ///     (u8, u16, u32, u64, i8, i16, i32, i64)
    /// );
    /// ```
    ///
    /// See also: [concat_many()], [TupleConcatMany].
    #[cfg_attr(docsrs, doc(cfg(feature = "concat-many")))]
    pub type ConcatMany<Tpls> = <Tpls as TupleConcatMany<Tpls>>::Type;

    /// Concatenate two or more tuples.
    ///
    /// ```
    /// use tupleops::concat_many;
    ///
    /// assert_eq!(
    ///     concat_many(()),
    ///     (),
    /// );
    ///
    /// assert_eq!(
    ///     concat_many(((1, 2, 3),)),
    ///     (1, 2, 3),
    /// );
    ///
    /// assert_eq!(
    ///     concat_many(((), (1,), (2, 3,), (4, 5, 6))),
    ///     (1, 2, 3, 4, 5, 6),
    /// );
    /// ```
    ///
    /// See also: [ConcatMany], [TupleConcatMany].
    #[cfg_attr(docsrs, doc(cfg(feature = "concat-many")))]
    #[inline(always)]
    pub fn concat_many<Tpls>(tpls: Tpls) -> ConcatMany<Tpls>
    where
        Tpls: TupleConcatMany<Tpls>,
    {
        <Tpls as TupleConcatMany<Tpls>>::concat_many(tpls)
    }

    /// A tuple of tuples that is usable with [concat_many()].
    ///
    /// See also: [concat_many], [TupleConcatMany].
    #[cfg_attr(docsrs, doc(cfg(feature = "concat-many")))]
    pub trait TupleConcatMany<Tpls> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn concat_many(tpls: Tpls) -> Self::Type;
    }

    impl TupleConcatMany<()> for () {
        type Type = ();

        #[inline(always)]
        fn concat_many(tpls: ()) -> Self::Type {
            let () = tpls;
        }
    }

    impl<I1> TupleConcatMany<(I1,)> for (I1,) {
        type Type = I1;

        #[inline(always)]
        fn concat_many(tpls: (I1,)) -> Self::Type {
            let (tpl,) = tpls;
            tpl
        }
    }

    impl<Z1, Z2> TupleConcatMany<(Z1, Z2)> for (Z1, Z2)
    where
        (Z1, Z2): TupleConcat<Z1, Z2>,
    {
        type Type = ConcatTuples<Z1, Z2>;

        #[inline(always)]
        fn concat_many(tpls: (Z1, Z2)) -> Self::Type {
            let (z1, z2) = tpls;
            concat_tuples(z1, z2)
        }
    }
});
