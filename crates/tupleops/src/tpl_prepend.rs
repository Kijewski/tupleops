crate::do_impl!("prepend", tuple_prepend, {
    /// The resulting type when an element is prepended to a tuple.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::Prepend;
    ///
    /// assert_same_types!(
    ///     Prepend<u8, (u16, u32)>,
    ///     (u8, u16, u32),
    /// );
    /// ```
    ///
    /// See also: [prepend()], [TuplePrepend].
    #[cfg_attr(docsrs, doc(cfg(feature = "prepend")))]
    pub type Prepend<Head, Tail> = <(Head, Tail) as TuplePrepend<Head, Tail>>::Type;

    /// Prepend an elemenent to a tuple.
    ///
    /// ```
    /// use tupleops::prepend;
    ///
    /// assert_eq!(
    ///     prepend(1, (2, 3, 4)),
    ///     (1, 2, 3, 4),
    /// );
    /// ```
    ///
    /// See also: [Prepend], [TuplePrepend].
    #[cfg_attr(docsrs, doc(cfg(feature = "prepend")))]
    #[inline(always)]
    pub fn prepend<Head, Tail>(head: Head, tail: Tail) -> Prepend<Head, Tail>
    where
        (Head, Tail): TuplePrepend<Head, Tail>,
    {
        <(Head, Tail) as TuplePrepend<Head, Tail>>::prepend(head, tail)
    }

    /// An element and a tuple that are usable with [prepend()].
    ///
    /// See [prepend()].
    ///
    /// See also: [prepend()], [Prepend].
    #[cfg_attr(docsrs, doc(cfg(feature = "prepend")))]
    pub trait TuplePrepend<Head, Tail> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn prepend(head: Head, tail: Tail) -> Self::Type;
    }

    impl<Head> TuplePrepend<Head, ()> for (Head, ()) {
        type Type = (Head,);

        #[inline(always)]
        fn prepend(head: Head, tail: ()) -> Self::Type {
            let () = tail;
            (head,)
        }
    }
});
