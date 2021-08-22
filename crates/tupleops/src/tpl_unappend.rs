crate::do_impl!("unappend", tuple_unappend, {
    /// The resulting tuple when the last element is removed from a tuple.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::Init;
    ///
    /// assert_same_types!(
    ///     Init<(u8, u16, u32)>,
    ///     (u8, u16),
    /// );
    /// ```
    ///
    /// See also: [unappend()], [Last], [TupleUnappend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unappend")))]
    pub type Init<Tpl> = <Tpl as TupleUnappend<Tpl>>::Init;

    /// The type of the last element of the tuple.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::Last;
    ///
    /// assert_same_types!(
    ///     Last<(u8, u16, u32)>,
    ///     u32,
    /// );
    /// ```
    ///
    /// See also: [unappend()], [Init], [TupleUnappend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unappend")))]
    pub type Last<Tpl> = <Tpl as TupleUnappend<Tpl>>::Last;

    /// Extract the last element of a tuple, and return a tuple of the initial tuple and the last
    /// element.
    ///
    /// ```
    /// use tupleops::unappend;
    ///
    /// assert_eq!(
    ///     unappend((1, 2, 3, 4)),
    ///     ((1, 2, 3), 4),
    /// );
    /// ```
    ///
    /// See also: [Init], [Last], [TupleUnappend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unappend")))]
    #[inline(always)]
    pub fn unappend<Tpl>(tpl: Tpl) -> (Init<Tpl>, Last<Tpl>)
    where
        Tpl: TupleUnappend<Tpl>,
    {
        <Tpl as TupleUnappend<Tpl>>::unappend(tpl)
    }

    /// A tuple that is usable with [unappend()].
    ///
    /// See also: [unappend()], [Last], [TupleUnappend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unappend")))]
    pub trait TupleUnappend<Tpl> {
        #[doc(hidden)]
        type Init;

        #[doc(hidden)]
        type Last;

        #[doc(hidden)]
        fn unappend(tpl: Tpl) -> (Self::Init, Self::Last);
    }

    impl<Elem> TupleUnappend<(Elem,)> for (Elem,) {
        type Init = ();
        type Last = Elem;

        #[inline(always)]
        fn unappend(tpl: (Elem,)) -> (Self::Init, Self::Last) {
            let (last,) = tpl;
            ((), last)
        }
    }
});
