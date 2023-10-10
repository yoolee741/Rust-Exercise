// 컴파일러 경고 끄기
#![allow(dead_code)]

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {

  //  let string = String::from("127.0.0.1:8080");

// 보통 문자열 슬라이스 시 아래와 같이 직접 자르는 경우가 없음 ([] 내의 숫자들은 바이트 기준임) -> 영어나 숫자와 같은 기호들과는 달리 특정 언어들이나 이모지는 UTF-8로 변환X, 각 문자가 1바이트일거란 확신 없기 때문에 
// 예를들면 find() 같은 함수가 있어서 문자열 안의 공백을 찾아낼 수 있음 -> 인덱스 반환해줌
 //   let string_slice = &string[10..];

 //   let string_borrow: &str = &string;

 //   let string_literal = "1234";


    // string을 디버깅하는 것은 에러가 발생하는데, 그 이유는 바로 위에 줄에서 이미 borrowing을 했기때문에 변수를 옮길 수 없음
   // dbg!(string);

    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);


    // 아래의 Server -> 구조체 (사용자 지정 데이터 타입)
    // to_string() 을 붙여줌으로써 아래의 로컬호스트 주소를 실제 문자열로 변환하고, HEAP의 다른 어딘가에 할당하게 됨
   let server = Server::new("127.0.0.1:8080".to_string());
   server.run(WebsiteHandler);
}










/*
* method 
-> 항상 self라는 첫번째 파라미터를 취함 (self = 구조체의 인스턴스)
-> 일반적인 함수와 비슷하나 구조체의 맥락 속에서 정의된다는 점이 다름

* 연관함수 (associated function)
-> 구조체 유형, 즉 이름에 연관된 함수를 의미
-> 구조체의 인스턴스 필요 X
-> 다른 언어의 정적 함수와 비슷
ex) new 키워드 (구조체 타입에 바로 호출하고 있기 때문) -> :: 구문을 사용하여 연관함수에 엑세스
 */ 

/*
* str
-> 문자열 슬라이스 -> 문자열 중 일부만 조회 및 사용할 경우
-> 어떤 문자열에 대한 immutable한 참조
-> 기존 문자열 안쪽에 있는 하나의 뷰
-> stack에 저장되는 형식 (아래의 요소는 힙 내에 있는 문자열 관련 요소)
    -> length
    -> ptr (pointer; 조회하고자 하는 문자 인덱스 주소)
    => 원하는 슬라이스의 길이와 기존 문자열에 대한 포인터만 저장
    => 변경을 하지 않고, 읽기나 조회만 하는 경우 효율적!

* String -> 전체 문자열
-> stack에 저장되는 형식 (아래의 요소는 힙 내에 있는 문자열 관련 요소)
-> 런타임에서 역동적으로 확장되거나, 수축될 수 있음 (그러나, str slice 는 immutable)
    -> length
    -> capacity
    -> ptr (pointer; 문자의 첫번째 글자의 주소)
 */