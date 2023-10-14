mod parse;
mod parse_payload; pub use parse_payload::*;
mod from_request;  pub use from_request::*;

#[cfg(test)] mod _parse_test;

use std::{borrow::Cow};
use percent_encoding::{percent_decode};
use crate::{
    __dep__,
    layer0_lib::{List, Method, Buffer, ContentType, Slice}
};


pub(crate) const QUERIES_LIMIT: usize = 4;
pub(crate) const HEADERS_LIMIT: usize = 32;

pub struct Request {
    _buffer: Buffer,
    method:  Method,
    path:    Slice,
    queries: List<(Slice, Slice), QUERIES_LIMIT>,
    headers: List<(Slice, Slice), HEADERS_LIMIT>,
    payload: Option<(ContentType, Slice)>,
} const _: () = {
    unsafe impl Send for Request {}
    unsafe impl Sync for Request {}
};

impl Request {
    pub(crate) async fn new(stream: &mut __dep__::TcpStream) -> Self {
        let buffer = Buffer::new(stream).await;
        parse::parse(buffer)
    }
}

impl Request {
    #[inline] pub fn method(&self) -> Method {
        self.method
    }
    #[inline] pub fn path(&self) -> &str {
        unsafe {std::mem::transmute(
            &*(percent_decode(self.path.into_bytes()).decode_utf8_lossy())
        )}
    }
    #[inline] pub fn query<Value: FromBuffer>(&self, key: &str) -> Option<Result<Value, Cow<'static, str>>> {
        for (key_, value) in self.queries.iter() {
            if key.eq_ignore_ascii_case(&percent_decode(unsafe {key_.into_bytes()}).decode_utf8_lossy()) {
                return Some(Value::parse((&percent_decode(unsafe {value.into_bytes()}).decode_utf8_lossy()).as_bytes()))
            }
        }
        None
    }
    #[inline] pub fn header(&self, key: &str) -> Option<&str> {
        for (key_, value) in self.headers.iter() {
            if key.as_bytes().eq_ignore_ascii_case(unsafe {key_.into_bytes()}) {
                return Some(unsafe {std::str::from_utf8_unchecked(value.into_bytes())})
            }
        }
        None
    }
    #[inline] pub fn payload(&self) -> Option<(&ContentType, &[u8])> {
        let (content_type, body) = (&self.payload).as_ref()?;
        Some((
            content_type,
            unsafe {body.into_bytes()},
        ))
    }
}

impl Request {
    #[inline(always)] pub(crate) fn path_bytes(&self) -> &[u8] {
        unsafe {self.path.into_bytes()}
    }
}

const _: () = {
    impl std::fmt::Debug for Request {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let queires = {
                let List { list, next } = &self.queries;
                list[..*next].into_iter()
                    .map(|cell| {
                        let (k, v) = unsafe {cell.assume_init_ref()};
                        format!("{} = {}",
                            percent_decode(unsafe {k.into_bytes()}).decode_utf8_lossy(),
                            percent_decode(unsafe {v.into_bytes()}).decode_utf8_lossy(),
                        )
                    })
            }.collect::<Vec<_>>();

            let headers = {
                let List { list, next } = &self.headers;
                list[..*next].into_iter()
                    .map(|cell| unsafe {
                        let (k, v) = cell.assume_init_ref();
                        format!("{}: {}",
                            std::str::from_utf8_unchecked(k.into_bytes()),
                            std::str::from_utf8_unchecked(v.into_bytes()),
                        )
                    })
            }.collect::<Vec<_>>();

            if let Some((_, payload)) = self.payload() {
                f.debug_struct("Request")
                    .field("method",  &self.method)
                    .field("path",    &self.path())
                    .field("queries", &queires)
                    .field("headers", &headers)
                    .field("payload", &String::from_utf8_lossy(payload))
                    .finish()

            } else {
                f.debug_struct("Request")
                    .field("method",  &self.method)
                    .field("path",    &self.path())
                    .field("queries", &queires)
                    .field("headers", &headers)
                    .finish()
            }
        }
    }
};




#[cfg(test)]
struct DebugRequest {
    method: Method,
    path: &'static str,
    queries: &'static [(&'static str, &'static str)],
    headers: &'static [(&'static str, &'static str)],
    payload: Option<(ContentType, &'static str)>,
}
#[cfg(test)]
const _: () = {
    impl DebugRequest {
        pub(crate) fn assert_parsed_from(self, req_str: &'static str) {
            let DebugRequest { method, path, queries, headers, payload } = self;
            let req = parse::parse(Buffer::from_raw_str(req_str));

            assert_eq!(req.method(), method);
            assert_eq!(req.path(), path);
            assert_eq!(req.payload().map(|(ct, s)| (ct.clone(), std::str::from_utf8(s).unwrap())), payload);
            for (k, v) in queries {
                assert_eq!(req.query::<String>(k), Some(Ok((*v).to_owned())))
            }
            for (k, v) in headers {
                assert_eq!(req.header(k), Some(*v))
            }
        }
    }
};
