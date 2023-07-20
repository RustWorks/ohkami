#![allow(non_snake_case, unused_mut)]

use crate::{
    Ohkami,
    layer3_fang_handler::{Handlers, ByAnother},
    layer4_router::{TrieRouter},
};


trait Rounting {
    fn apply(self, routes: TrieRouter) -> TrieRouter;
} const _: () = {
    impl Rounting for Handlers {
        fn apply(self, routes: TrieRouter) -> TrieRouter {
            routes.register_handlers(self)
        }
    }
    impl Rounting for ByAnother {
        fn apply(self, routes: TrieRouter) -> TrieRouter {
            routes.merge_another(self)
        }
    }
};

macro_rules! build_routing {
    ($( $routing_item:ident ),*) => {
        impl<$($routing_item: Rounting),*> FnOnce<($($routing_item,)*)> for Ohkami {
            type Output = Ohkami;
            extern "rust-call" fn call_once(mut self, ($($routing_item,)*): ($($routing_item,)*)) -> Self::Output {
                $(
                    self.routes = $routing_item.apply(self.routes);
                )*
                self
            }
        }
    };
} const _: () = {
    build_routing!();
    build_routing!(R1);
    build_routing!(R1, R2);
    build_routing!(R1, R2, R3);
    build_routing!(R1, R2, R3, R4);
    build_routing!(R1, R2, R3, R4, R5);
    build_routing!(R1, R2, R3, R4, R5, R6);
    build_routing!(R1, R2, R3, R4, R5, R6, R7);
};