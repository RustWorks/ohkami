use std::sync::Arc;
use super::Ohkami;
use crate::{__dep__, Request, Context};
#[cfg(feature="rt_async-std")] use crate::__dep__::StreamExt;


pub trait TCPAddress {
    fn parse(self) -> String;
} const _: () = {
    impl TCPAddress for u16 {
        fn parse(self) -> String {
            if self > 49151 {panic!("port must be 0 〜 49151")}
            format!("0.0.0.0:{self}")
        }
    }
    impl TCPAddress for &'static str {
        fn parse(self) -> String {
            if self.starts_with(":") {
                format!("0.0.0.0{self}")
            } else if self.starts_with("localhost") {
                self.replace("localhost", "127.0.0.1")
            } else {
                self.to_owned()
            }
        }
    }
};


impl Ohkami {
    pub async fn howl(self, address: impl TCPAddress) {
        let router = Arc::new({
            let mut routes = self.routes;
            for fang in self.fangs {
                routes = routes.apply_fang(fang)
            }
            routes.into_radix()
        });
        
        let listener = match __dep__::TcpListener::bind(address.parse()).await {
            Ok(listener) => listener,
            Err(e)       => panic!("Failed to bind TCP listener: {e}"),
        };

        #[cfg(feature="rt_async-std")]
        while let Some(Ok(mut stream)) = listener.incoming().next().await {
            let router = Arc::clone(&router);
            let c = Context::new();

            __dep__::task::spawn(async move {
                let req = Request::new(&mut stream).await;
                router.handle(c, req, stream).await;
            }).await;
        }
        
        #[cfg(feature="rt_tokio")]
        loop {
            let mut stream = match listener.accept().await {
                Ok((stream, _)) => stream,
                Err(e)          => panic!("Failed to bind TCP listener: {e}"),
            };

            let router = Arc::clone(&router);
            let c = Context::new();

            if let Err(e) = __dep__::task::spawn(async move {
                let req = Request::new(&mut stream).await;
                router.handle(c, req, stream).await;
            }).await {
                panic!("Fatal error: {e}")
            }
        }
    }
}