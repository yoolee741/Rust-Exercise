fn main() {
    /*
    * Ownership Rules
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
     
     */

   // ()
   // {
        // s is not valid here. it's not yet declared
       // let s = "hello" // s is valid from this point forward
        // string literal directly stored to binary

        let s: String = String::from("hello"); // now string stored on a heap -> when we declare, rust automatically allocates memory on the heap for a string


        
        // do stuff with s
        
   // } // this scope is now over, and s is no longer valid

let x = 5;
let y = x; // copy

let s1: String = String::from("hello");
// let s2 = s1; // Move (not shallow copy)
// println!("{}, world!", s1) <- error occurred (value borrowed here after move)

// what if we want to clone, rather than move?
let s2:String = s1.clone();
println!("{}, world!", s1);

let s = String::from("hello");
takes_ownership(s);
// println!("{}", s);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}