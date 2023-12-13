extern crate add_one;
extern crate rand;


use add_one::add_one;

fn main() {
    let num  = 10;

    // println!("Hello, world! {} plus one is {}", num, add_one::add_one(num));
    println!("Hello, world! {} plus one is {}", num, add_one(num));
}
