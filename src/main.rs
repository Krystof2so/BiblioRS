// Ne contient plus que la logique principale

mod all_struct;
mod input;

use all_struct::book::Book;

fn main() {
    let _test = 5;
    println!("{}", Book::new_book().display());
}
