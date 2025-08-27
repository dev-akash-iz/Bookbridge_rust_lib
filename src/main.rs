use crate::book_bridge::{load_binary, split_it};

mod book_bridge;


pub fn main(){
    load_binary();

    split_it("".to_string(),"".to_string());
}