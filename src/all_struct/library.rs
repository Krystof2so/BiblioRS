use super::book::Book; // Import de la structure `Book` depuis le module parent

/// Structure représentant une bibliothèque de livres.
#[derive(Debug)]
pub struct Library {
    books: Vec<Book>, // Vecteur stockant les instances de `Book`
}

/// Implémentation des méthodes pour la structure `Library`.
impl Library {
    /// Crée une nouvelle instance de `Library` vide.
    pub fn new() -> Self {
        Library { books: Vec::new() } // Initialise un vecteur vide pour stocker les livres
    }

    /// Ajoute un livre à la bibliothèque.
    /// `book` - Une instance de `Book` à ajouter à la bibliothèque.
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book); // Ajoute le livre au vecteur `books`
    }

    /// Affiche chaque livre
    pub fn display_all(&self) {
        for book in &self.books {
            println!(
                "Clé : {}\nAuteur : {} {}\nTitre : {}\nAnnée : {}\nPages : {}\n---",
                book.id_book(),
                book.author_name,
                book.author_first_name,
                book.title,
                book.pub_year,
                // Nombre de pages, sans les zéros initiaux
                // Cf. https://doc.rust-lang.org/std/primitive.str.html#method.trim_start_matches
                book.nb_pages.trim_start_matches('0').to_string()
            );
        }
    }
}
