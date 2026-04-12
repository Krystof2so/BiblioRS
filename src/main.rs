//! Point d'entrée du binaire : ne contient que la logique principale
//! Permet à l'utilisateur d'ajouter des livres à une bibliothèque de façon interactive.

use rs_biblio::input::user_input::build_book;
use rs_biblio::models::library::Library;
use rs_biblio::utils::read_input;

fn main() {
    let mut my_library = Library::new();

    loop {
        println!("Souhaitez-vous ajouter un livre à votre bibliothèque ? (o/n)");
        if read_input().to_lowercase() != "o" {
            break;
        }

        // La saisie et la validation des champs sont gérées dans build_book()
        let book = build_book();

        println!("\nInformations saisies :\n{}", book.display());
        println!("Les informations sont-elles correctes ? (o/n)");
        if read_input().to_lowercase() != "o" {
            // TODO : Implémenter une possible correction ?
            continue;
        }

        my_library.add_book(book);
        println!("Enregistré dans la bibliothèque");

        // LIGNES DE TEST
        println!("\n\nContenu de la structure 'Library':\n{:?}", my_library);
    }
}
