fn main() {
    // variables -> mut를 붙여줘야 reassign 가능
    let mut x = 5;
    println!("The value of x is : {}", x);

    x = 14;
    println!("The value of x is : {}", x);

    x = 4;
    println!("The value of x is : {}", x);

    // constant -> 비록 let 이 이미 immutable이지만 상수와 변수에는 차이가 있어서 구분을 해주는 것
    // 상수의 경우 타입을 지정하고 나면 리턴값의 타입 변경 불가

    const SUBSCRIBER_COUNT: u32 =100000;

    // shadow: shadowing allows you to create a new variable using an existing name
    // mut를 없애도 still mutable
    let y = 7;
    println!("The value of y is : {}", y);

    let y = "seven";
    println!("The value of y is : {}", y);

    /* 
    scalar data type = a single value
    1. Integers -> 정수 (보통은 32bit)
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    let f: u8 = 255; // u8은 255가 맥시멈


    2. Floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;


    // addition
    let sum = 5 + 10;

   // subtraction
   let difference: 95.5 - 4.3;

   // multiplication
   let product = 4 * 30;

   // division
   let quotient = 56.7 / 32.2;

   // remainder
   let remainder = 43 % 5;

    3. Booleans

    let t = true;
    let f = false;

     
    4. Character
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = 'cat';

    */ 

    /* 
    compound type = a group of value

    let tup = ("Let's Get Rusty!", 100_000); -> tup = tuple

    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    let error_codes = [200, 404,500];
    let not_found = error_codes[1];
    let x = error_codes[2];


    let byte = [0; 8]; -> create integer all set to zero
    */ 

  let sum = my_function(11, 22); 
  println!("The value of sum is : {}", sum);

  let controlFlow = control_flow();
  
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another Function");
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
    let sum = x + y;
    return sum;
}

fn control_flow () {
    let number = 5;
    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
        
    }
    

// in rust, the condition must be explicitly a boolean
    let condition = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("control_flow is: {}", number);

    let mut counter = 3;
   let result = loop {
       counter+=1;
       if counter == 10 {
        break counter;
       }
    };

    println!("The result is {}", result);

    let mut num = 3;
    while num != 0 {
        println!("The num is {}", num);
        num -= 1;
    };
    println!("Finish!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the for loop value is {}", element);
    }

    for e in (1..4) {
        println!("the for loop value is {}", e);
    }

}