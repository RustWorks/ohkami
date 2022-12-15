use std::{str::{self, Lines}, ops::{Index, Range}};
use async_std::{net::TcpStream, io::ReadExt};
use crate::{result::Result, components::method::Method};
use super::{map::RangeMap, parse::parse_stream_lines};

const BUF_SIZE: usize = 1024;

pub(crate) struct Buffer(
    [u8; BUF_SIZE]
); impl Buffer {
    pub async fn new(stream: &mut TcpStream) -> Result<Self> {
        let mut buffer = [b' '; BUF_SIZE];
        stream.read(&mut buffer).await?;
        Ok(Self(buffer))
    }
    pub fn lines(&self) -> Result<Lines> {
        Ok(str::from_utf8(&self.0)?.lines())
    }
    // pub fn parse(
    //     &self
    // ) -> Result<(Method, &str, RangeMap, RangeMap)> {
    //     parse_stream_lines(self.lines()?)
    // }
}

impl Index<Range<usize>> for Buffer {
    type Output = [u8];
    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}
