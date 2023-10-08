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
        }
    
    
        }
    