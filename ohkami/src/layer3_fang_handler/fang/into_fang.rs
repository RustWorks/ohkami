#![allow(non_snake_case)]

use std::{sync::Arc, any::Any};
use crate::{
    Context,
    Request,
    Response,
    layer3_fang_handler::{FrontFang, FangProc, BackFang, Fang},
};


pub trait IntoFang<Args> {
    /// Why Option: returns `None` when executing
    /// some builtin fangs like `cors`. In other words,
    /// this **internally executes** proc that returns `None`.
    fn into_fang(self) -> Option<Fang>;
}

const _: (/* Front: not retuning Result */) = {
    impl<F: Fn(&Context) + Send + Sync + 'static>
    IntoFang<(&Context,)> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |c, req| {
                        self(&c);
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&mut Context) + Send + Sync + 'static>
    IntoFang<(&mut Context,)> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |mut c, req| {
                        self(&mut c);
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&Request) + Send + Sync + 'static>
    IntoFang<(&Request,)> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |c, req| {
                        self(&req);
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&Context, &Request) + Send + Sync + 'static>
    IntoFang<(&Context, &Request)> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |c, req| {
                        self(&c, &req);
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&mut Context, &Request) + Send + Sync + 'static>
    IntoFang<(&mut Context, &Request)> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |mut c, req| {
                        self(&mut c, &req);
                        Ok((c, req))
                    }
                ))),
            })
        }
    }
};

const _: (/* Front: returning Result */) = {
    impl<F: Fn(&Context)->Result<(), Response> + Send + Sync + 'static>
    IntoFang<(&Context, ())> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |c, req| {
                        self(&c)?;
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&mut Context)->Result<(), Response> + Send + Sync + 'static>
    IntoFang<(&mut Context, ())> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |mut c, req| {
                        self(&mut c)?;
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&Request)->Result<(), Response> + Send + Sync + 'static>
    IntoFang<(&Request, ())> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |c, req| {
                        self(&req)?;
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl<F: Fn(&Context, &Request)->Result<(), Response> + Send + Sync + 'static>
    IntoFang<(&Context, &Request, ())> for F {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |c, req| {
                        self(&c, &req)?;
                        Ok((c, req))
                    }
                ))),
            })
        }
    }

    impl IntoFang<(&mut Context, &Request, ())> for fn(&mut Context, &Request) -> Result<(), Response> {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Front(FrontFang(Arc::new(
                    move |mut c, req| {
                        self(&mut c, &req)?;
                        Ok((c, req))
                    }
                ))),
            })
        }
    }
};

const _: (/* Back */) = {
    impl IntoFang<(&Response,)> for fn(&Response) {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Back(BackFang(Arc::new(
                    move |res| {
                        self(&res);
                        res
                    }
                ))),
            })
        }
    }

    impl IntoFang<(&mut Response,)> for fn(&mut Response) {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Back(BackFang(Arc::new(
                    move |mut res| {
                        self(&mut res);
                        res
                    }
                ))),
            })
        }
    }

    impl IntoFang<(&Response, ())> for fn(&Response) -> Result<(), Response> {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Back(BackFang(Arc::new(
                    move |res| {
                        match self(&res) {
                            Ok(_)  => res,
                            Err(e) => e,
                        }
                    }
                ))),
            })
        }
    }

    impl IntoFang<(&mut Response, ())> for fn(&mut Response) -> Result<(), Response> {
        fn into_fang(self) -> Option<Fang> {
            Some(Fang {
                id:   self.type_id(),
                proc: FangProc::Back(BackFang(Arc::new(
                    move |mut res| {
                        match self(&mut res) {
                            Ok(_)  => res,
                            Err(e) => e,
                        }
                    }
                ))),
            })
        }
    }
};