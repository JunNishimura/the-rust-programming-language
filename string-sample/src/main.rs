fn main() {
    let data = "initial contents";
    let s = data.to_string();

    println!("{}", s);

    let s2 = String::from("string 2");
    println!("{}", s2);

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("{}", s3);
    println!("{}", s4);

    let s5 = String::from("hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6;
    println!("{}", s7);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let t = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", t);

    let s8 = String::from("‚±‚ñ‚É‚¿‚Í");
    for c in s8.chars() {
	println!("{}", c);
    }
}
