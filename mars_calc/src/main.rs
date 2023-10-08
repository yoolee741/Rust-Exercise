// https://doc.rust-lang.org/stable/std/io/index.html
use std::io;

fn main() {
    
    // some_fn(&input);
    // some_fn_mut(&mut input);

    println!("Enter your weight (kg)!: ");
    let mut input = String::new();
 // 콘솔에서 입력되는 값으로 Input이 채워질 것
 io::stdin().read_line(&mut input).unwrap();
 println!("Input: {}", input);

    // trim -> 공백제거, parse -> 원하는 타입으로 변환
    let weight: f32 = input.trim().parse().unwrap();
    println!("{}", weight);

     // 디버그
    // dbg!("{}", weight);

   

// borrow_string(&input);
// own_string(input);

    let mut mars_weight = calculate_weight_on_mars(weight);
    
   println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
// parameter -> weight on earth
 (weight / 9.81) * 3.711
}

// & -> 참조 (immutable)
// fn some_fn (s: &String) {

// }

// &mut -> 가변 참조
// fn some_fn_mut (s: &mut String) {
//     s.push_str("a");
// }

// borrow string
// fn borrow_string(s: &String) {
//     println!("{}", s);
// }

// own string
// fn own_string(s: String) {
//     println!("{}", s);
// }