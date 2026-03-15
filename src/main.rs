// Module principal : ne contient plus que la logique principale
// Permet à l'utilisateur d'ajouter des livres à une bibliothèque et d'afficher leur contenu.

// Import des modules nécessaires
mod all_struct; // Module contenant les structures de données (Book, Library, etc.)
mod input; // Module contenant la logique de saisie utilisateur

// Import des structures utilisées
use all_struct::{book::Book, library::Library};
use input::check_input::ask_year;

fn main() {
    let mut my_library = Library::new(); // Initialise une bibliothèque vide

    // Boucle principale pour ajouter des livres
    loop {
        println!("Souhaitez-vous ajouter un livre à votre bibliothèque ? (o/n)");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Erreur de lecture");
        // Si la réponse n'est pas "o" (oui), quitte la boucle
        if choice.trim().to_lowercase() != "o" {
            break;
        }

        // Crée un nouveau livre en demandant les informations à l'utilisateur
        let book = Book::new_book();
        // Affiche les informations du livre nouvellement créé
        println!("{}", book.display());
        // Ajoute le livre à la bibliothèque
        my_library.add_book(book);
        println!("Enregistré dans la bibliothèque");
    }

    // Affichages
    println!("\n--- Bibliothèque complète (par date de parution) ---");
    my_library.sort_by_year(); // Tri par dates de parution
    my_library.display_all(); // Affichage complet
    // Affichage selon une année de référence
    my_library.display_from_year(ask_year("\nLivres parus après l'année : "));
}
