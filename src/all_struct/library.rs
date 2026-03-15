use super::book::Book; // Import de la structure `Book` depuis le module parent

/// Structure représentant une bibliothèque de livres.
#[derive(Debug, Clone)]
pub struct Library {
    pub books: Vec<Book>, // Vecteur stockant les instances de `Book`
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
            println!("{}", Book::display2(book));
        }
    }

    pub fn sort_by_year(&mut self) {
        self.books.sort_by_key(|book| book.pub_year);
    }

    pub fn display_from_year(&mut self, ref_year: i32) {
        self.sort_by_year();
        let books_from_year: Vec<&Book> = self
            .books
            .iter()
            .filter(|book| book.pub_year >= ref_year)
            .collect();
        print!("\n--- Liste des livres parus à partir de {ref_year} ---\n");
        for book in &books_from_year {
            println!("{}", Book::display2(book));
        }
    }
}
