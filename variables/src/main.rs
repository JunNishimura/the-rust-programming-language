fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");

    another_function();
    another_function2(10);

    let x = five();
    println!("The value of x is: {}", x);
}

fn another_function() {
    print!("Another function");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}
