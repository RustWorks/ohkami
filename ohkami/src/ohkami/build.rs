#![allow(non_snake_case, unused_mut)]

use super::router::TrieRouter;
use crate::handler::{Handlers, ByAnother};


trait RoutingItem {
    fn apply(self, router: &mut TrieRouter);
} const _: () = {
    impl RoutingItem for Handlers {
        fn apply(self, router: &mut TrieRouter) {
            router.register_handlers(self)
        }
    }
    impl RoutingItem for ByAnother {
        fn apply(self, router: &mut TrieRouter) {
            router.merge_another(self)
        }
    }

    /// This is for better developer experience.
    /// 
    /// If we impl `Routes` only for `Handlers` and `ByAnother`, ohkami users
    /// will see following situations：
    /// 
    /// ```ignore
    /// fn my_ohkami() -> Ohkami {
    ///     Ohkami::new((
    ///         "/".|
    /// /*          ↑ cursor */
    ///     ))
    /// }
    /// 
    /// // Here rust-analyzer puts red underlines for all lines of `Ohkami::new(( 〜 ))`
    /// // because the type of argument of `new` is `&str` **AT NOW** and `Routes` trait is
    /// // NOT IMPLEMENTED for this.
    /// // 
    /// // This must be so annoying!!!
    /// ```
    impl RoutingItem for &'static str {
        fn apply(self, _router: &mut TrieRouter) {}
    }
};

pub trait Routes {
    fn apply(self, router: &mut TrieRouter);
} impl<R: RoutingItem> Routes for R {
    fn apply(self, router: &mut TrieRouter) {
        <R as RoutingItem>::apply(self, router)
    }
} macro_rules! impl_for_tuple {
    ( $( $item:ident ),+ ) => {
        impl<$( $item: RoutingItem ),+> Routes for ( $($item,)+ ) {
            fn apply(self, router: &mut TrieRouter) {
                let ( $( $item, )+ ) = self;
                $(
                    <$item as RoutingItem>::apply($item, router);
                )+
            }
        }
    };
} const _: () = {
    impl Routes for () {fn apply(self, _router: &mut TrieRouter) {}}
    
    impl_for_tuple!(R1);
    impl_for_tuple!(R1, R2);
    impl_for_tuple!(R1, R2, R3);
    impl_for_tuple!(R1, R2, R3, R4);
    impl_for_tuple!(R1, R2, R3, R4, R5);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15);
    impl_for_tuple!(R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, R14, R15, R16);
};