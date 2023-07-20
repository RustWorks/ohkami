use crate::{
    layer3_fang_handler::{IntoFang, Fang},
};


pub trait Fangs<G>: Sized {
    fn collect(self) -> Vec<Fang>;
} const _: () = {
    impl Fangs<()> for () {
        fn collect(self) -> Vec<Fang> {
            vec![]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1>
        Fangs<Args1> for F1
    where
        F1: IntoFang<Args1>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1>
        Fangs<(Args1,)> for (F1,)
    where
        F1: IntoFang<Args1>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.0.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1, F2, Args2>
        Fangs<(Args1, Args2)> for (F1, F2)
    where
        F1: IntoFang<Args1>,
        F2: IntoFang<Args2>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.0.into_fang(), self.1.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1, F2, Args2, F3, Args3>
        Fangs<(Args1, Args2, Args3)> for (F1, F2, F3)
    where
        F1: IntoFang<Args1>,
        F2: IntoFang<Args2>,
        F3: IntoFang<Args3>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.0.into_fang(), self.1.into_fang(), self.2.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1, F2, Args2, F3, Args3, F4, Args4>
        Fangs<(Args1, Args2, Args3, Args4)> for (F1, F2, F3, F4)
    where
        F1: IntoFang<Args1>,
        F2: IntoFang<Args2>,
        F3: IntoFang<Args3>,
        F4: IntoFang<Args4>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.0.into_fang(), self.1.into_fang(), self.2.into_fang(), self.3.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1, F2, Args2, F3, Args3, F4, Args4, F5, Args5>
        Fangs<(Args1, Args2, Args3, Args4, Args5)> for (F1, F2, F3, F4, F5)
    where
        F1: IntoFang<Args1>,
        F2: IntoFang<Args2>,
        F3: IntoFang<Args3>,
        F4: IntoFang<Args4>,
        F5: IntoFang<Args5>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.0.into_fang(), self.1.into_fang(), self.2.into_fang(), self.3.into_fang(), self.4.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }

    impl<F1, Args1, F2, Args2, F3, Args3, F4, Args4, F5, Args5, F6, Args6>
        Fangs<(Args1, Args2, Args3, Args4, Args5, Args6)> for (F1, F2, F3, F4, F5, F6)
    where
        F1: IntoFang<Args1>,
        F2: IntoFang<Args2>,
        F3: IntoFang<Args3>,
        F4: IntoFang<Args4>,
        F5: IntoFang<Args5>,
        F6: IntoFang<Args6>,
    {
        fn collect(self) -> Vec<Fang> {
            vec![self.0.into_fang(), self.1.into_fang(), self.2.into_fang(), self.3.into_fang(), self.4.into_fang(), self.5.into_fang()]
                .into_iter()
                .filter_map(|of| of)
                .collect()
        }
    }
};