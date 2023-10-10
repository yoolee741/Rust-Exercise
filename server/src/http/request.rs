use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Formatter, Display, Debug, Result as FmtResult};
use std::str::{self, Utf8Error};
use super::method::{Method, MethodError};
use super::{QueryString};


// derive()가 들어있는 대괄호 안에 컴파일러가 우릴 대신해서 유도해주길 원하는 트레이트를 지정 -> 디버깅 가능 (보통 #뒤에 !가 없으면 해당 라인 다음에 나오는 줄에 반영)
#[derive(Debug)]


pub struct Request<'buf> {
          path: &'buf str,
          query_string: Option<QueryString<'buf>>,
          method: super::method::Method
}

// Request 내에 있는 필드들이 모두 private 이기 때문에 모듈 밖에서는 접근이 어려움 -> getter 구현
impl<'buf> Request <'buf> {
    pub fn path(&self) -> &str {
        &self.path
    } 

    pub fn method(&self) -> &Method {
        &self.method
    }

    //query_string는 &Option에는 관심이 없고, Option이 감싸고 있는 것에만 관심이 있음
    // 이 Option 안에 있는 데이터에 대한 참조와 함께 Option을 리턴하는 것이 더 합리적
    // .as_ref() -> converts from &Option<T> to Option<&T>
    // 이런식으로 메서드를 구현하면 훨씬 더 유연 -> query_string에 저장한 타입을 변경할 수 있고, getter에서 option으로 감쌀 수 있음
    
    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        // 오류처리 방법 #1.
        // match str::from_utf8(buf) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding)
        // }

        // 오류처리 방법 #2.
        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e)
        // }


        // 오류처리 방법 #3.
        // ? 는 from_utf8() 함수의 Result를 보고 만일 Result가 오류라면 가서 이 함수의 예상되는 Result Error에 대한 From 트레이트의 구현체와 실제로 받는 오류 체크
        let request = str::from_utf8(buf)?;


        // Option을 Result로 변환하기 (match get_next_word -> let (method, request))
        // match get_next_word(request) {
        //     Some(method, request) => {}
        //     None => return Err(ParseError::InvalidRequest)
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })


        // 아직 구현할 준비가 되지 않은 어떠한 함수에도 그 unimplemented1() 매크로 호출 가능
       // unimplemented!();
    }
}


impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
// 메소드와 쿼리 분리하는 함수
fn get_next_word(request: &str) -> Option<(&str, &str)>{
    
   for (i, c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
        // 문자열에선 i+1 과 같은 형식을 사용하는것이 매우 위험하나, 우리의 경우 분리 기준이 공백 (즉, 1바이트)인 것을 알기에 예외적으로 +1 처리로 해줌
        return Some((&request[..i], &request[i + 1..]));
    }

   }
   None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
} 

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod"
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}