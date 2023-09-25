fn main() {
    println!("hello rust!");
}

/*
과정:
1. 기능을 담은 rs 파일을 작성한다.
2. 터미널에 rustc 파일이름.rs 을 입력 후 실행시키면 동일한 이름의 컴파일 파일이 생성된다.
3. 터미널에 ./{컴파일 파일이름} 입력하면 해당 파일 내에 있는 함수 작동

예시:
1. main.rs 생성
2. rustc main.rs -> main 파일 생성
3. ./main
4. 결과물 => hello rust!
 */