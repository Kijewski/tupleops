crate::do_impl!("all-ok", tuple_all_ok, {
    /// The type when a tuple of [Result]s is element-wise unwrapped.
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::AllOk;
    ///
    /// assert_same_types!(
    ///     AllOk<(Result<u8, ()>, Result<u16, ()>)>,
    ///     (u8, u16),
    /// );
    /// ```
    ///
    /// See also: [all_ok()], [TupleAllOk].
    #[cfg_attr(docsrs, doc(cfg(feature = "all-ok")))]
    pub type AllOk<Tpl> = <Tpl as TupleAllOk<Tpl>>::Type;

    /// Element-wise unwrap a tuple of [Result]s if all elements are good.
    /// Return the input otherwise.
    ///
    /// ```
    /// use tupleops::all_ok;
    ///
    /// fn good(value: i32) -> Result<i32, ()> {
    ///     Ok(value)
    /// }
    ///
    /// fn bad(_: i32) -> Result<i32, ()> {
    ///     Err(())
    /// }
    ///
    /// assert_eq!(
    ///     all_ok((good(1), good(2), good(3))),
    ///     Ok((1, 2, 3)),
    /// );
    ///
    /// assert_eq!(
    ///     all_ok((good(1), bad(2), good(3))),
    ///     Err((Ok(1), Err(()), Ok(3))),
    /// );
    ///
    /// assert_eq!(
    ///     all_ok(()),
    ///     Ok(()),
    /// );
    /// ```
    ///
    /// See also: [AllOk], [TupleAllOk].
    #[cfg_attr(docsrs, doc(cfg(feature = "all-ok")))]
    #[inline(always)]
    pub fn all_ok<Tpl>(tpl: Tpl) -> Result<AllOk<Tpl>, Tpl>
    where
        Tpl: TupleAllOk<Tpl>,
    {
        <Tpl as TupleAllOk<Tpl>>::all_ok(tpl)
    }

    /// A tuple that is usable with [all_ok()].
    ///
    /// See also: [all_ok()], [TupleAllOk].
    #[cfg_attr(docsrs, doc(cfg(feature = "all-ok")))]
    pub trait TupleAllOk<Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn all_ok(tpl: Tpl) -> Result<Self::Type, Tpl>;
    }

    impl TupleAllOk<()> for () {
        type Type = ();

        #[doc(hidden)]
        fn all_ok(tpl: ()) -> Result<Self::Type, ()> {
            let () = tpl;
            Ok(())
        }
    }
});
