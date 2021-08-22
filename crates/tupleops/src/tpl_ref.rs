crate::do_impl!("ref", tuple_ref, {
    /// The resulting type when every element of this reference to a tuple is turned into a
    /// reference.
    ///
    /// ```ignore
    /// use same_types::assert_same_types;
    /// use tupleops::RefTuple;
    ///
    /// assert_same_types!(
    ///     RefTuple<'a, (u8, u16, u32)>,
    ///     (&'a u8, &'a u16, &'a u32),
    /// );
    /// ```
    ///
    /// See also: [ref_tuple()], [TupleRef].
    #[cfg_attr(docsrs, doc(cfg(feature = "ref")))]
    pub type RefTuple<'a, Tpl> = <&'a Tpl as TupleRef<'a, Tpl>>::Type;

    /// Turn a reference to a tuple into a tuple of references.
    ///
    /// ```
    /// use tupleops::ref_tuple;
    ///
    /// assert_eq!(
    ///     ref_tuple(&(1, 2, 3)),
    ///     (&1, &2, &3),
    /// );
    /// ```
    ///
    /// See also: [RefTuple], [TupleRef].
    #[cfg_attr(docsrs, doc(cfg(feature = "ref")))]
    #[inline(always)]
    pub fn ref_tuple<'a, Tpl>(tpl: &'a Tpl) -> RefTuple<'a, Tpl>
    where
        &'a Tpl: TupleRef<'a, Tpl>,
    {
        <&'a Tpl as TupleRef<'a, Tpl>>::ref_tuple(tpl)
    }

    /// A reference to a tuple that is usable with [ref_tuple()].
    ///
    /// See also: [ref_tuple()], [RefTuple].
    #[cfg_attr(docsrs, doc(cfg(feature = "ref")))]
    pub trait TupleRef<'a, Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn ref_tuple(tpl: &'a Tpl) -> Self::Type;
    }

    impl<'a> TupleRef<'a, ()> for &'a () {
        type Type = ();

        #[inline(always)]
        fn ref_tuple((): &'a ()) -> Self::Type {}
    }
});
