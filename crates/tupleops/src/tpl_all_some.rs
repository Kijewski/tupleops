crate::do_impl!("all-some", tuple_all_some, {
    /// The type when a tuple of [Option]s is element-wise unwrapped.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::AllSome;
    ///
    /// assert_same_types!(
    ///     AllSome<(Option<u8>, Option<u16>, Option<u32>)>,
    ///     (u8, u16, u32),
    /// );
    /// ```
    ///
    /// See also: [all_some()], [TupleAllSome].
    #[cfg_attr(docsrs, doc(cfg(feature = "all-some")))]
    pub type AllSome<Tpl> = <Tpl as TupleAllSome<Tpl>>::Type;

    /// Element-wise unwrap a tuple of [Option]s if all elements are good.
    /// Return the input otherwise.
    ///
    /// ```
    /// use tupleops::all_some;
    ///
    /// assert_eq!(
    ///     all_some((Some(1), Some(2), Some(3))),
    ///     Ok((1, 2, 3)),
    /// );
    ///
    /// assert_eq!(
    ///     all_some((Some(1), Option::<i32>::None, Some(3))),
    ///     Err((Some(1), None, Some(3))),
    /// );
    ///
    /// assert_eq!(
    ///     all_some(()),
    ///     Ok(()),
    /// );
    /// ```
    ///
    /// See also: [AllSome], [TupleAllSome].
    #[cfg_attr(docsrs, doc(cfg(feature = "all-some")))]
    #[inline(always)]
    pub fn all_some<Tpl>(tpl: Tpl) -> Result<AllSome<Tpl>, Tpl>
    where
        Tpl: TupleAllSome<Tpl>,
    {
        <Tpl as TupleAllSome<Tpl>>::all_some(tpl)
    }

    /// A tuple that is usable with [all_some()].
    ///
    /// See also: [all_some()], [TupleAllSome].
    #[cfg_attr(docsrs, doc(cfg(feature = "all-some")))]
    pub trait TupleAllSome<Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn all_some(tpl: Tpl) -> Result<Self::Type, Tpl>;
    }

    impl TupleAllSome<()> for () {
        type Type = ();

        #[doc(hidden)]
        fn all_some(tpl: ()) -> Result<Self::Type, ()> {
            let () = tpl;
            Ok(())
        }
    }
});
