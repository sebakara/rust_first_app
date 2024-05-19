use std::io;

fn main() {
//    let x = 4;
//     println!("the value of x is: {}", x);

//     {
//         let x = "hello mavin";
//         println!("x here is: {}", x);
//     }

//     let x = x + 1;
//     println!("now x is: {}", x);

//     // tupple
//     let tup:(i32, bool, char) = (34, true, 'h');
//     println!("let us see tupple: {}", tup.2);

//     // use array
//     let mut arr = [1,3,2,4,5];

//     arr[4] = 7;
//     println!("{}", arr[4]);

//     let y = 5;
//     let p = y;

//     println!("{},{}", y, p);

//     // inputs
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).expect("failed to read line");
//     println!("this input has: {}", input);

//     let mut input2 = String::new();
//     io::stdin().read_line(&mut input2).expect("expected and integer");
//     let int_input2 :i64 = input2.trim().parse().unwrap();
//     println!("conversions is: {}", int_input2 + 2);

//     // arthmetics

//     let x = 7;
//     let y = 5;

//     let r:i32 = 127;
//     let k:i8 = 8;

//     let q = k as i32 + r;
//     let z = x + y;
//     let p = x % y;
//     println!("addition is {}, modulo is {}, add different types {} ", z, p, q); 

//     // conditions

//     let cond = 2 < 3;
//     println!("{}", cond);

//     // if and else if

//     let food = "cookie";
//     if food != "cookie" {
//         println!("I like cookies too");
//     } else if food == "potates" {
//         println!("ohhhhh, not a big fan");
//     } else {
//         println!("that's too bad");
//     }

// functions
let sum = addtion(23, 5);
println!("the sum is {}", sum);
fn addtion (x: i32, y: i32) -> i32 {
   return x + y;
}

}

