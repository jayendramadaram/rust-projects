use std::io;
use std::cmp::Ordering::{Equal , Greater , Less};
use rand::Rng;

fn main() {
    let specialnum : i32 = rand::thread_rng().gen_range(1..99);
    loop {
        println!("U gotta enter stuff \t");
        let mut usernum = String::new();
        io::stdin().read_line(&mut usernum).expect("Failed to read Line");
        // print!("Enterd stuff {}" , username);
        let usernum : i32 =match usernum.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Brooo!! failed");
                continue;
            }
        };
        match usernum.cmp(&specialnum) {
            Equal => {
                println!("You got that right buddy");
                break;
            },
            Less => println!("You got that less buddy"),
            Greater => println!("You got that more buddy"),
        }
        println!()
    }
}
