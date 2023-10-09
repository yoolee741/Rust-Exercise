use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;

// 모듈화 -> 기본적으로 모듈 내에 있는 요소들은 다 private
// 데이터를 담고 있는 블록 = 구조체
pub struct Server {
     addr: String,
}
    
// 모든 실제 기능을 담고 있는 그 구조체의 구현 블록을 만듦 -> 구현 블록을 만들기 위해 impl이라는 키워드 사용
impl Server {
    // 우리에게 필요한 기능 나열
    pub fn new(addr: String) -> Server {
        Server {
            addr: addr
            }
    }
    
    //메서드
    pub fn run(self) {
        println!("Listening on : {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

       // 반복문 : loop, while, for
       // match는 enum 뿐 아니라 switch 구문처럼도 사용 가능
       loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                // 러스트에는 요소들의 값이 똑같은 배열을 생성하기 위한 구문이 존재 -> 초기값 설정
                // [0; 1024] -> [채우고 싶은 요소 (이 경우엔 0으로 채우고 싶음); 배열의 길이]
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(_) => {
                        // 유입되는 요청을 로깅하는 로직
                        // String::from_utf8() 과 String::from_utf8_lossy() 의 차이점
                        // String::from_utf8() : 바이트가 담긴 버퍼를 파라미터로 예상하며, 그 바이트는 유효한 utf-8이어야 함 
                        // (유효하지 않은 경우 연산 전체 실패 -> Result 타입을 리턴하며 여기에는 유효하지 않은 utf-8 바이트가 포함되어 있을 수 있기 때문)
                        // String::from_utf8_lossy() : String::from_utf8()와 비슷한 기능이나, 절대로 실패하지 않음 -> 바이트 슬라이스를 유효하지 않은 문자도 포함하여 문자열로 변환
                        println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                        match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                dbg!(request);
                               let response = Response::new(StatusCode::Ok, Some("<h1>IT WORKS!</h1>".to_string()));
                               write!(stream, "{}", response);
                            }
                            Err(e) => {println!("Failed to parse a request: {}", e);}
                        }
                       // let res: &Result<Request, _> = &buffer[..].try_into()
                    }
                    Err(e) => println!("Failed to read from connection: {}", e)
                }
            }

            Err(e) => {
                println!("Failed to establish a connection: {}", e);
            }
        }
   
       }

    }
    
    
}

/*
* Rust는 loop의 레이블을 지정할 수 있음
-> 안쪽 loop의 본문에서 바깥쪽 loop를 break || continue 하려고 한다면, 레이블을 이용하여 loop에 주석을 달 수 있음 -> '를 쓴 후 원하는 레이블을 적어주면 됨

* 예시:
 'outer: loop {
           loop {
               break 'outer;

               // 안쪽 loop의 본문에서 바깥쪽 loop를 break하려고 한다면, 레이블을 이용하여 loop에 주석을 달 수 있음 -> '를 쓴 후 원하는 레이블을 적어주면 됨
           }
       }
 */

/*
* 튜플
-> 길이가 정해져 있고, 한번 선언한 다음에는 크기를 늘리거나 줄일 수 없음
-> 서로 다른 타입들 묶음 가능
예: let tup = (5, "a", listener);
 */

/*
* match
예시:
match "abcd" {
    "abcd" => println!(),
    "a" | "b" => {}
    _ => {} // default 값 
}
 */