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
}
