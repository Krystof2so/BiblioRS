use super::super::input::user_input::user_entry;
use super::book::Book; // Import de la structure `Book` depuis le module parent

/// Structure représentant une bibliothèque de livres.
#[derive(Debug)]
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

    /// Trier les livres selon la date de parution
    pub fn sort_by_year(&mut self) {
        self.books.sort_by_key(|book| book.pub_year);
    }

    /// Affichage selon une année de référence (à l'aide de .filter)
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

    /// Compter les références par auteur
    pub fn instances_by_author(&self) {
        let author_query =
            user_entry("Quel auteur recherchez-vous (indiquez son nom) ? ", false).to_uppercase();
        // Comptage des occurrences avec match et les méthodes .count() et .filter()
        let instances = self
            .books
            .iter()
            .filter(|book| book.author_name.to_uppercase() == author_query)
            .count(); // count() compte le nombre d'éléments dans l'itérateur
        match instances {
            0 => println!(
                "Aucun livre de {} n'a été trouvé dans la bibliothèque",
                author_query
            ),
            _ => println!(
                "Il y a {} livres de {} dans la bibliothèque",
                instances, author_query
            ),
        }
    }
}
