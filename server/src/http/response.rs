
use std::{fmt::{Formatter, Display, Result as FmtResult}, net::TcpStream};
use std::io::{Write, Result as IoResult};
use super::{StatusCode, Request};


#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}

// 기존에 write를 사용해 formatter에 기록한 것은 동일한 값을 새로운 문자열에 할당함으로써 heap에도 저장하게됨 => 불필요한 메모리 자원을 썼었음
// 동일한 매크로 (write)를 사용하고 있지만 fn send라는 함수에 직접적으로 매크로를 넣음으로써 TcpStream에 직접 기록 => 할당 필요X

// dyn => dynamic (다이내믹 디스패치)
// impl -> static dispatch => 컴파일 시 해당 함수 전체를 훓으면서 각 타입에 맞게 함수를 복사
