
    // enum variant 들은 서로 다른 타입으로 지정해주는 것도 가능
   // 예: GET(String), POST(i32), PUT = 5
   pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    PATCH,
    TRACE
    }
   