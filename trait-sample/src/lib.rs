use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Debug
{
    0
}

struct Pair<T> {
	x: T,
	y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
	Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
	if self.x >= self.y {
	    println!("The largest member is x = {}", self.x);
	} else {
	    println!("The largest member is y = {}", self.y);
	}
	}
}
 
