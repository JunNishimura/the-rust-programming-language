use std::{
    fs::File, 
    io::{
	Error,
	ErrorKind,
	Read,
    },
};

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
	Ok(file) => file,
	Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
	Ok(_) => Ok(s),
	Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_very_short() -> Result<String, Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
	Ok(file) => file,
	Err(ref error) if error.kind() == ErrorKind::NotFound => {
	    match File::create("hello.txt") {
		Ok(fc) => fc,
		Err(e) => {
		    panic!("Tried to create file but there was a problem: {:?}", e)
		},
	    }
	},
	Err(error) => {
	    panic!("There was a problem opening the file: {:?}", error)
	}
    };

    // let f = File::open("hello2.txt").unwrap();
    let f = File::open("hello3.txt").expect("Failed to open hello3.txt");
}
