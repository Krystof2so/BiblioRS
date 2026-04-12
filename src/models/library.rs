//! Définition de la structure [`Library`] et de ses méthodes.

use super::book::Book;
use std::collections::HashMap;

/// Collection de livres indexée par leur identifiant unique.
///
/// La clé est l'identifiant généré par [`Book::id_book`].
#[derive(Debug)]
pub struct Library {
    pub books: HashMap<String, Book>,
}

impl Library {
    /// Crée une bibliothèque vide.
    pub fn new() -> Self {
        Library {
            books: HashMap::new(),
        }
    }

    /// Ajoute un livre à la bibliothèque.
    ///
    /// La clé est générée automatiquement via [`Book::id_book`].
    /// Prend possession du `book` (transfert de propriété, pas de copie).
    pub fn add_book(&mut self, book: Book) {
        let id = book.id_book();
        self.books.insert(id, book);
    }
}
