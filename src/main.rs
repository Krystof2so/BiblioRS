// Ne contient plus que la logique principale

mod all_struct;
mod check_input;
mod input;

use all_struct::book::Book;
use all_struct::book::ask_s;
use back_of_house::test::abc;

fn main() {
    let _test = 5;
    println!("{}", Book::new_book().display());
    ask_s("Test question", true);
    back_of_house::_fix_incorrect_order();
    abc();
}
pub fn test() {
    println!("Test !");
}

pub fn deliver_order() {
    println!("test !!")
}

mod back_of_house;
