use crate::book_bridge::{load_binary, split_it};

mod book_bridge;


pub fn main(){
    load_binary(&"".to_string());

    split_it("C:\\Users\\akash.v\\RustroverProjects\\untitled\\pkpadmin,+529-2711-1-CE.pdf".to_string(),"C:\\Users\\akash.v\\RustroverProjects\\untitled\\generated".to_string());
}