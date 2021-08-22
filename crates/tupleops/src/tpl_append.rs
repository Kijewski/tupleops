crate::do_impl!("append", tuple_append, {
    /// The resulting type when an element is appended to an initial tuple.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::Append;
    ///
    /// assert_same_types!(
    ///     Append<(u8, u16), u32>,
    ///     (u8, u16, u32),
    /// );
    /// ```
    ///
    /// See also: [append()], [TupleAppend].
    #[cfg_attr(docsrs, doc(cfg(feature = "append")))]
    pub type Append<Init, Last> = <(Init, Last) as TupleAppend<Init, Last>>::Type;

    /// Append an element to a tuple.
    ///
    /// ```
    /// use tupleops::append;
    ///
    /// assert_eq!(
    ///     append((1, 2, 3), 4),
    ///     (1, 2, 3, 4),
    /// );
    /// ```
    ///
    /// See also: [Append], [TupleAppend].
    #[cfg_attr(docsrs, doc(cfg(feature = "append")))]
    #[inline(always)]
    pub fn append<Init, Last>(init: Init, last: Last) -> Append<Init, Last>
    where
        (Init, Last): TupleAppend<Init, Last>,
    {
        <(Init, Last) as TupleAppend<Init, Last>>::append(init, last)
    }

    /// A tuple and an element that are usable with [append()].
    ///
    /// See also: [append()], [TupleAppend].
    #[cfg_attr(docsrs, doc(cfg(feature = "append")))]
    pub trait TupleAppend<Init, Last> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn append(init: Init, last: Last) -> Self::Type;
    }

    impl<Last> TupleAppend<(), Last> for ((), Last) {
        type Type = (Last,);

        #[inline(always)]
        fn append(init: (), last: Last) -> Self::Type {
            let () = init;
            (last,)
        }
    }
});
