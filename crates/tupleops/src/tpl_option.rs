crate::do_impl!("option", tuple_option, {
    /// The resulting tuple when all elements are wrapped in [Option].
    ///
    /// ```
    /// use same_types::assert_same_types;
    /// use tupleops::OptionTuple;
    ///
    /// assert_same_types!(
    ///     OptionTuple<(u8, u16, u32)>,
    ///     (Option<u8>, Option<u16>, Option<u32>),
    /// );
    /// ```
    ///
    /// See also: [option_tuple()], [TupleOption].
    #[cfg_attr(docsrs, doc(cfg(feature = "option")))]
    pub type OptionTuple<Tpl> = <Tpl as TupleOption<Tpl>>::Type;

    /// Element-wise wrap the element of a tuple in [Option].
    ///
    /// ```
    /// use tupleops::option_tuple;
    ///
    /// assert_eq!(
    ///     option_tuple(Some((1, 2, 3))),
    ///     (Some(1), Some(2), Some(3)),
    /// );
    ///
    /// assert_eq!(
    ///     option_tuple(Option::<(u8, u16, u32)>::None),
    ///     (None, None, None),
    /// );
    /// ```
    ///
    /// See also: [OptionTuple], [TupleOption].
    #[cfg_attr(docsrs, doc(cfg(feature = "option")))]
    #[inline(always)]
    pub fn option_tuple<Tpl>(tpl: Option<Tpl>) -> OptionTuple<Tpl>
    where
        Tpl: TupleOption<Tpl>,
    {
        <Tpl as TupleOption<Tpl>>::option_tuple(tpl)
    }

    /// A tuple that is usable with [option_tuple()].
    ///
    /// See also: [option_tuple()], [OptionTuple].
    #[cfg_attr(docsrs, doc(cfg(feature = "option")))]
    pub trait TupleOption<Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn option_tuple(tpl: Option<Tpl>) -> Self::Type;
    }

    impl TupleOption<()> for () {
        type Type = ();

        fn option_tuple(tpl: Option<()>) -> Self::Type {
            let _ = tpl;
        }
    }
});
