use super::book::Book;

#[derive(Debug)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn display_all(&self) {
        for book in &self.books {
            println!(
                "Clé : {}\nAuteur : {} {}\nTitre : {}\nAnnée : {}\nPages : {}\n---",
                book.id_book(),
                book.author_name,
                book.author_first_name,
                book.title,
                book.pub_year,
                book.nb_pages.trim_start_matches('0').to_string() // https://doc.rust-lang.org/std/primitive.str.html#method.trim_start_matches
            );
        }
    }
}
