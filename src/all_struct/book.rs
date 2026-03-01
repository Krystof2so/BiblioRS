use crate::check_input::book::{ask_pages, ask_year};
use crate::input::user_input::ask_string;
pub use crate::input::user_input::ask_string as ask_s;

#[derive(Debug)]
pub struct Book {
    pub author_name: String,
    pub author_first_name: String,
    pub title: String,
    pub pub_year: i32,
    nb_pages: String,
}

impl Book {
    fn only_4_chars(my_string: &str) -> String {
        my_string.chars().take(4).collect::<String>().to_uppercase()
    }

    fn _id_book(&self) -> String {
        format!(
            "{}{}{}{}",
            Self::only_4_chars(&self.author_name),
            self.pub_year,
            Self::only_4_chars(&self.title),
            self.nb_pages,
        )
    }

    pub fn display(&self) -> String {
        format!(
            "{}, de {} {}, a été publié en {}\nClé d'identification : {}",
            self.title,
            self.author_first_name,
            self.author_name.to_uppercase(),
            self.pub_year,
            self._id_book()
        )
    }

    pub fn new_book() -> Book {
        Book {
            author_name: ask_string("Nom de l'auteur : ", true),
            author_first_name: ask_string("Prénom de l'auteur : ", false),
            title: ask_string("Titre du livre : ", true),
            pub_year: ask_year("Année de publication : "),
            nb_pages: ask_pages("Nombre de pages : "),
        }
    }
}
