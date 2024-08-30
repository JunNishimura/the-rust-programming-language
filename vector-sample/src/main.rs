enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];
    let mut mv = Vec::new();
    mv.push(5);
    mv.push(6);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);
    
    match v2.get(2) {
	Some(third) => println!("The third element is {}", third),
	None => println!("There is no third element."),
    }

    let first = &mv[0];
    //mv.push(6);
    println!("The first element is: {}", first);

    let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Text(String::from("blue")),
	SpreadsheetCell::Float(10.12),
    ];
}

