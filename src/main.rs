use std::io;

fn main() {
   let x = 4;
    println!("the value of x is: {}", x);

    {
        let x = "hello mavin";
        println!("x here is: {}", x);
    }

    let x = x + 1;
    println!("now x is: {}", x);

    // tupple
    let tup:(i32, bool, char) = (34, true, 'h');
    println!("let us see tupple: {}", tup.2);

    // use array
    let mut arr = [1,3,2,4,5];

    arr[4] = 7;
    println!("{}", arr[4]);

    let y = 5;
    let p = y;

    println!("{},{}", y, p);

    // inputs
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("this input has: {}", input);
}
