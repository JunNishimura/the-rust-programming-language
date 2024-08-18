use std::fmt::Display;

fn main() {
    let r;
    {
	let x = 5;
	r = &x;
	println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    let result;
    let result2;
    {
	let string4 = String::from("xyz");
	result = longest(string3.as_str(), string4.as_str());
	println!("The longest string is {}", result);
	
	result2 = longest2(string3.as_str(), string4.as_str());
    }

    // println!("The longest string is {}", result);
    println!("The longest string is {}", result2);

    let ann = "Announcement";

    let result = longest_with_announcement(string1.as_str(), string2, ann);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
	x
    } else {
	y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
	x
    } else {
	y
    }
}
    
    
