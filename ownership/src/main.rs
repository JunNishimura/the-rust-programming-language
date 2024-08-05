fn main() {
    let s1 = String::from("hello");

    take_ownership(&s1);

    println!("{}", s1);

    let mut s2 = String::from("hello");    

    change(&mut s2);

    println!("{}", s2);

    let s3 = String::from("hello world");

    let w1 = &s3[0..5];
    let w2 = &s3[6..11];

    println!("{} {}", w1, w2);  
}

fn take_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn change(s: &mut String) {
    s.push_str(", world");
}