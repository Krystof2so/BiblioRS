// Ne contient plus que la logique principale

mod all_struct;
mod input;

use all_struct::{book::Book, library::Library};

fn main() {
    let mut library = Library::new(); // bibliothèque vide

    loop {
        println!("Souhaitez-vous ajouter un livre à votre bibliothèque ? (o/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Erreur de lecture");
        if choice.trim().to_lowercase() != "o" {
            break;
        }

        let book = Book::new_book();
        println!("{}", book.display());
        library.add_book(book);
        println!("Enregistré dans la bibliothèque");
    }

    println!("\n--- Bibliothèque complète ---");
    library.display_all();
}
