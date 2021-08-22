crate::do_impl!("apply", tuple_apply, {
    /// The resulting type when F is called with Tpl's elements.
    ///
    /// See also: [apply()], [TupleApply].
    #[cfg_attr(docsrs, doc(cfg(feature = "apply")))]
    pub type Apply<F, Tpl> = <(F, Tpl) as TupleApply<F, Tpl>>::Type;

    /// Call a function with the tuples members as arguments.
    ///
    /// The function F can be <code>&[Fn]</code> or <code>&mut [FnMut]</code>.
    ///
    /// ```
    /// use tupleops::apply;
    ///
    /// fn add3(a: u32, b: u32, c: u32) -> u32 {
    ///     a + b + c
    /// }
    ///
    /// let tpl3 = (1, 2, 3);
    /// assert_eq!(apply(&add3, tpl3), 6);
    /// ```
    ///
    /// ```
    /// use tupleops::apply;
    ///
    /// let mut counter = 0;
    /// let mut prefixsum3 = |a, b, c| {
    ///     counter += a + b + c;
    ///     counter
    /// };
    ///
    /// assert_eq!(apply(&mut prefixsum3, (1, 2, 3)), 6);
    /// assert_eq!(apply(&mut prefixsum3, (4, 5, 6)), 21);
    /// assert_eq!(apply(&mut prefixsum3, (7, 8, 9)), 45);
    /// ```
    ///
    /// See also: [Apply], [TupleApply].
    #[cfg_attr(docsrs, doc(cfg(feature = "apply")))]
    #[inline(always)]
    pub fn apply<F, Tpl>(func: F, tpl: Tpl) -> Apply<F, Tpl>
    where
        (F, Tpl): TupleApply<F, Tpl>,
    {
        <(F, Tpl) as TupleApply<F, Tpl>>::apply(func, tpl)
    }

    /// A function and a tuple that are usable with [apply()].
    ///
    /// See also: [apply()], [Apply].
    #[cfg_attr(docsrs, doc(cfg(feature = "apply")))]
    pub trait TupleApply<F, Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn apply(func: F, tpl: Tpl) -> Self::Type;
    }

    impl<F> TupleApply<&F, ()> for (&F, ()) {
        type Type = ();

        fn apply(_func: &F, _tpl: ()) -> Self::Type {}
    }

    impl<F> TupleApply<&mut F, ()> for (&mut F, ()) {
        type Type = ();

        fn apply(_func: &mut F, _tpl: ()) -> Self::Type {}
    }
});
