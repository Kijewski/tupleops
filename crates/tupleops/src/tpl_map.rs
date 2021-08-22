crate::do_impl!("map", tuple_map, {
    /// The type of a tuple when element-wise mapped with a mapper.
    ///
    /// See also: [map_tuple()], [TupleMapper], [TupleMap].
    pub type MapTuple<Mapper, Tpl> = <(Mapper, Tpl) as TupleMap<Mapper, Tpl>>::Type;

    /// Element-wise map a tuple with a mapper.
    ///
    /// ```
    /// #![feature(generic_associated_types)]
    ///
    /// use tupleops::{TupleMapper, map_tuple};
    ///
    /// struct MyTupleEnum(usize);
    ///
    /// impl TupleMapper for MyTupleEnum {
    ///     type MapElem<Type> = (usize, Type);
    ///
    ///     fn map_elem<Elem>(&mut self, elem: Elem) -> Self::MapElem<Elem> {
    ///         let index = self.0;
    ///         self.0 += 1;
    ///         (index, elem)
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     map_tuple(MyTupleEnum(1), ("hello", "world", "!")),
    ///     ((1, "hello"), (2, "world"), (3, "!")),
    /// )
    /// ```
    ///
    /// See also: [MapTuple], [TupleMapper], [TupleMap].
    #[cfg_attr(docsrs, doc(cfg(feature = "map")))]
    #[inline(always)]
    pub fn map_tuple<Mapper: TupleMapper, Tpl>(mapper: Mapper, tpl: Tpl) -> MapTuple<Mapper, Tpl>
    where
        (Mapper, Tpl): TupleMap<Mapper, Tpl>,
    {
        <(Mapper, Tpl) as TupleMap<Mapper, Tpl>>::map_tuple(mapper, tpl)
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "map")))]
    /// Helper trait to element-wise map a tuple.
    ///
    /// See also: [map_tuple()], [MapTuple], [TupleMap].
    pub trait TupleMapper {
        /// The result type after mapping Elem.
        type MapElem<Elem>;

        /// Map an element.
        fn map_elem<Elem>(&mut self, elem: Elem) -> Self::MapElem<Elem>;
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "map")))]
    /// A [TupleMapper] and a tuple that are usable with [map_tuple()].
    ///
    /// See also: [map_tuple()], [MapTuple], [TupleMapper].
    pub trait TupleMap<Mapper: TupleMapper, Tpl> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn map_tuple(mapper: Mapper, tpl: Tpl) -> Self::Type;
    }

    impl<Mapper: TupleMapper> TupleMap<Mapper, ()> for (Mapper, ()) {
        type Type = ();

        fn map_tuple(_mapper: Mapper, _tpl: ()) -> Self::Type {}
    }
});
