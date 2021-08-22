crate::do_impl!("ref-mut", tuple_ref_mut, {
    /// The resulting type when every element of this reference to a mutable tuple is turned into
    /// a mutable reference.
    ///
    /// ```ignore
    /// use same_types::assert_same_types;
    /// use tupleops::RefMutTuple;
    ///
    /// assert_same_types!(
    ///     RefMutTuple<'a, (u8, u16, u32)>,
    ///     (&'a mut u8, &'a mut u16, &'a mut u32),
    /// );
    /// ```
    ///
    /// See also: [ref_mut_tuple()], [TupleRefMut].
    #[cfg_attr(docsrs, doc(cfg(feature = "ref-mut")))]
    pub type RefMutTuple<'a, Tpl> = <&'a mut Tpl as TupleRefMut<'a, Tpl>>::Type;

    /// Turn a reference to a mutable tuple into a tuple of mutable references.
    ///
    /// ```
    /// use tupleops::ref_mut_tuple;
    ///
    /// assert_eq!(
    ///     ref_mut_tuple(&mut (1, 2, 3)),
    ///     (&mut 1, &mut 2, &mut 3),
    /// );
    /// ```
    ///
    /// See also: [RefMutTuple], [TupleRefMut].
    #[cfg_attr(docsrs, doc(cfg(feature = "ref-mut")))]
    #[inline(always)]
    pub fn ref_mut_tuple<'a, Tpl>(tpl: &'a mut Tpl) -> RefMutTuple<'a, Tpl>
    where
        &'a mut Tpl: TupleRefMut<'a, Tpl>,
    {
        <&'a mut Tpl as TupleRefMut<'a, Tpl>>::ref_mut_tuple(tpl)
    }

    /// A reference to a mutable tuple that is usable with [ref_mut_tuple()].
    ///
    /// See also: [ref_mut_tuple()], [RefMutTuple].
    #[cfg_attr(docsrs, doc(cfg(feature = "ref-mut")))]
    pub trait TupleRefMut<'a, Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn ref_mut_tuple(tpl: &'a mut Tpl) -> Self::Type;
    }

    impl<'a> TupleRefMut<'a, ()> for &'a mut () {
        type Type = ();

        #[inline(always)]
        fn ref_mut_tuple((): &'a mut ()) -> Self::Type {}
    }
});
