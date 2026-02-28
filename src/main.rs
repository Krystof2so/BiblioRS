// Ne contient plus que la logique principale

mod all_struct;
mod check_input;
mod input;

use all_struct::book::Book;

fn main() {
    println!("{}", Book::new_book().display());
}
