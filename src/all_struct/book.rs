use crate::input::check_input::{ask_pages as ask_p, ask_year as ask_y};
use crate::input::user_input::ask_string as ask_s;

#[derive(Debug, Clone)]
pub struct Book {
    pub author_name: String,
    pub author_first_name: String,
    pub title: String,
    pub pub_year: i32,
    pub nb_pages: String,
}

impl Book {
    /// Pour retourner 4 caractères en uppercase
    fn only_4_chars(my_string: &str) -> String {
        let mut four_chars: String = my_string
            .chars() // Itère sur la chaîne de caractères
            .take(4) // Prend les 4 premiers caractères (ou tous si moins de 4)
            .collect(); // Rassemble la chaîne en nouvelle String
        // Si la nouvelle String fait moins de 4 caractères, ajout de 'X' pour  obtenir 4 caractères
        while four_chars.len() < 4 {
            four_chars.push('X');
        }
        four_chars.replace(" ", "X").to_uppercase()
    }

    pub fn id_book(&self) -> String {
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
            self.id_book()
        )
    }

    pub fn display2(&self) -> String {
        format!(
            "Clé : {}\nAuteur : {} {}\nTitre : {}\nAnnée : {}\nPages : {}\n",
            self.id_book(),
            self.author_name,
            self.author_first_name,
            self.title,
            self.pub_year,
            self.nb_pages.trim_start_matches('0').to_string()
        )
    }

    pub fn new_book() -> Book {
        Book {
            author_name: ask_s("Nom de l'auteur : ", true, true),
            author_first_name: ask_s("Prénom de l'auteur : ", false, false),
            title: ask_s("Titre du livre : ", true, false),
            pub_year: ask_y("Année de publication : "),
            nb_pages: ask_p("Nombre de pages : "),
        }
    }
}
