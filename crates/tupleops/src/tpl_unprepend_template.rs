pub use r#impl::*;

#[cfg(not(feature = "unprepend-template"))]
mod r#impl {}

#[cfg(feature = "unprepend-template")]
mod r#impl {
    use crate::{append, unprepend, Append, Head, Tail, TupleAppend, TupleUnprepend};

    /// TODO
    pub type UnprependTemplate<Template, Tpl> =
        <(Template, (), Tpl) as TupleUnprependTemplate<Template, (), Tpl>>::Type;

    /// TODO
    ///
    /// ```
    /// use tupleops::unprepend_template;
    ///
    /// type Tmpl = (u8, u16, u32);
    /// assert_eq!(
    ///     unprepend_template::<Tmpl, _>((1, 2, 3, 4, 5, 6, 7)),
    ///     ((1, 2, 3), (4, 5, 6, 7)),
    /// );
    /// ```
    #[inline(always)]
    pub fn unprepend_template<Template, Tpl>(tpl: Tpl) -> UnprependTemplate<Template, Tpl>
    where
        (Template, (), Tpl): TupleUnprependTemplate<Template, (), Tpl>,
    {
        <(Template, (), Tpl) as TupleUnprependTemplate<_, _, _>>::unprepend_template((), tpl)
    }

    /// TODO
    pub trait TupleUnprependTemplate<Template, Left, Right> {
        #[doc(hidden)]
        type Type;

        #[doc(hidden)]
        fn unprepend_template(left: Left, right: Right) -> Self::Type;
    }

    impl<Left, Right> TupleUnprependTemplate<(), Left, Right> for ((), Left, Right) {
        type Type = (Left, Right);

        #[doc(hidden)]
        #[inline(always)]
        fn unprepend_template(left: Left, right: Right) -> Self::Type {
            (left, right)
        }
    }

    #[allow(clippy::type_complexity)]
    impl<Template, Left, Right> TupleUnprependTemplate<Template, Left, Right>
        for (Template, Left, Right)
    where
        Template: TupleUnprepend<Template>,
        Right: TupleUnprepend<Right>,
        (Left, Head<Right>): TupleAppend<Left, Head<Right>>,
        (Tail<Template>, Append<Left, Head<Right>>, Tail<Right>):
            TupleUnprependTemplate<Tail<Template>, Append<Left, Head<Right>>, Tail<Right>>,
    {
        type Type =
            <(Tail<Template>, Append<Left, Head<Right>>, Tail<Right>) as TupleUnprependTemplate<
                Tail<Template>,
                Append<Left, Head<Right>>,
                Tail<Right>,
            >>::Type;

        #[inline(always)]
        fn unprepend_template(left: Left, right: Right) -> Self::Type {
            let (head, new_right) = unprepend(right);
            let new_left = append(left, head);
            <(Tail<Template>, Append<Left, Head<Right>>, Tail<Right>) as TupleUnprependTemplate<
                _,
                _,
                _,
            >>::unprepend_template(new_left, new_right)
        }
    }
}
