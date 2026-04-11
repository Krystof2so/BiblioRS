//! Définition de la structure [`Library`] et de ses méthodes.

use super::book::Book;

/// Collection de livres.
///
/// Le champ `books` est public pour permettre une éventuelle itération
/// ou affichage depuis l'extérieur du module.
#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    /// Crée une bibliothèque vide.
    pub fn new() -> Self {
        Library { books: Vec::new() } // Initialise un vecteur vide pour stocker les livres
    }

    /// Ajoute un livre à la bibliothèque.
    ///
    /// Prend possession du `book` (transfert de propriété, pas de copie).
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
}
