crate::do_impl!("length", tuple_length, {
    /// Return the length of a tuple.
    ///
    /// ```
    /// use tupleops::length;
    ///
    /// assert_eq!(0, length(&()));
    /// assert_eq!(1, length(&(1,)));
    /// assert_eq!(2, length(&(1, 2)));
    /// assert_eq!(3, length(&(1, 2, 3)));
    /// ```
    ///
    /// See also: [TupleLength].
    #[cfg_attr(docsrs, doc(cfg(feature = "length")))]
    #[cfg_attr(docsrs, doc(cfg(feature = "feature-const_fn_trait_bound")))]
    #[cfg(feature = "feature-const_fn_trait_bound")]
    pub const fn length<Tpl>(_: &Tpl) -> usize
    where
        Tpl: TupleLength,
    {
        <Tpl as TupleLength>::LENGTH
    }

    /// A tuple with a known length.
    ///
    /// ```
    /// use tupleops::TupleLength;
    ///
    /// assert_eq!(0, <() as TupleLength>::LENGTH);
    ///
    /// assert_eq!(1, <(u8,) as TupleLength>::LENGTH);
    ///
    /// assert_eq!(2, <(u8, u16) as TupleLength>::LENGTH);
    ///
    /// assert_eq!(3, <(u8, u16, u32) as TupleLength>::LENGTH);
    /// ```
    ///
    /// See also: [length()].
    #[cfg_attr(docsrs, doc(cfg(feature = "length")))]
    pub trait TupleLength {
        /// The number of elements in this tuple
        const LENGTH: usize;
    }

    impl TupleLength for () {
        const LENGTH: usize = 0;
    }
});
