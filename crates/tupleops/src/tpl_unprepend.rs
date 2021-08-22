crate::do_impl!("unprepend", tuple_unprepend, {
    /// The type of the first element of the tuple.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::Head;
    ///
    /// assert_same_types!(
    ///     Head<(u8, u16, u32)>,
    ///     u8,
    /// );
    /// ```
    ///
    /// See also: [unprepend()], [Tail], [TupleUnprepend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unprepend")))]
    pub type Head<Tpl> = <Tpl as TupleUnprepend<Tpl>>::Head;

    /// The resulting tuple when the first element is removed from a tuple.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::Tail;
    ///
    /// assert_same_types!(
    ///     Tail<(u8, u16, u32)>,
    ///     (u16, u32),
    /// );
    /// ```
    ///
    /// See also: [unprepend()], [Tail], [TupleUnprepend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unprepend")))]
    pub type Tail<Tpl> = <Tpl as TupleUnprepend<Tpl>>::Tail;

    /// Extract the first element of a tuple, and return a tuple of the first element and the
    /// last elements of the tuple.
    ///
    /// ```
    /// use tupleops::unprepend;
    ///
    /// assert_eq!(
    ///     unprepend((1, 2, 3, 4)),
    ///     (1, (2, 3, 4)),
    /// );
    /// ```
    ///
    /// See also: [Head], [Tail], [TupleUnprepend].
    #[cfg_attr(docsrs, doc(cfg(feature = "unprepend")))]
    #[inline(always)]
    pub fn unprepend<Tpl>(tpl: Tpl) -> (Head<Tpl>, Tail<Tpl>)
    where
        Tpl: TupleUnprepend<Tpl>,
    {
        <Tpl as TupleUnprepend<Tpl>>::unprepend(tpl)
    }

    /// A tuple that is usable with [unprepend()].
    ///
    /// See also: [unprepend()], [Head], [Tail].
    #[cfg_attr(docsrs, doc(cfg(feature = "unprepend")))]
    pub trait TupleUnprepend<Tpl> {
        #[doc(hidden)]
        type Head;

        #[doc(hidden)]
        type Tail;

        #[doc(hidden)]
        fn unprepend(tpl: Tpl) -> (Self::Head, Self::Tail);
    }

    impl<Elem> TupleUnprepend<(Elem,)> for (Elem,) {
        type Head = Elem;
        type Tail = ();

        #[inline(always)]
        fn unprepend(tpl: (Elem,)) -> (Self::Head, Self::Tail) {
            let (head,) = tpl;
            (head, ())
        }
    }
});
