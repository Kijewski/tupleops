crate::do_impl!("tuple", tuple_tuple, {
    /// A tuple.
    #[cfg_attr(docsrs, doc(cfg(feature = "tuple")))]
    pub trait Tuple {}

    impl Tuple for () {}
});
