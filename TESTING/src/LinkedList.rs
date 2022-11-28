// LinkedList using enums

use std::fmt::format;

// use crate::List::NONE;
enum List {
    Prev(u32 , Box<List>),
    NONE,    
}

impl List {
    fn new() -> List {
        List::NONE
    }

    fn add(self , num : u32) -> List {
        List::Prev(num, Box::new(self))
    }

    fn len(&self) -> u32{
        match *self {
            List::Prev(_ , ref tail) => 1 + tail.len(),
            List::NONE  => 0
        }
    }

    fn printlis(&self) -> String {
        match *self {
            List::Prev(head, ref tail) => format!("{} {}" , head , tail.printlis()),
            List::NONE => String::from("NONE"),
        }
    }
}

pub fn main() {
    let mut LinkedList : List = List::new();
    LinkedList = LinkedList.add(45);
    LinkedList = LinkedList.add(45);
    LinkedList = LinkedList.add(24);
    LinkedList = LinkedList.add(245);
    LinkedList = LinkedList.add(45);

    let Str : String = LinkedList.printlis();
    println!("{}" , Str)
}
