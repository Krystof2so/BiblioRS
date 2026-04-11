//! Lecture, formatage des entrées texte et construction d'un [`Book`].

use crate::input::check_input::{ask_pages, ask_year};
use crate::models::book::Book;
use crate::utils::{read_input, remove_accents};

// **************
// * CONSTANTES *
// **************
// Les &str partagés et invariants sont déclarés `static` plutôt que `const` :
// `static` garantit une adresse mémoire unique.
// cf. https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
static EMPTY_FIELD: &str = "Ce champ ne peut être vide";

// ********************
// * FONCIONS PRIVEES *
// ********************

/// Met en forme une chaîne en "Title Case" compatible Unicode.
///
/// Seule la première lettre de chaque mot est mise en majuscule et désaccentuée
/// (via [`remove_accents`]) ; le reste du mot est conservé tel quel.
///
/// Exemple : `"île de ré"` → `"Ile De Re"`
fn to_title_case_unicode(title: &str) -> String {
    let mut vec_string = Vec::<String>::new();
    for word in title.split_whitespace() {
        // `chars()` itère sur des scalaires Unicode (et non des octets)
        let mut chars = word.chars();
        match chars.next() {
            // `chars.next()` consomme le premier scalaire ; `chars.as_str()`
            // retourne le reste de la tranche sans réallocation
            None => vec_string.push(String::new()),
            Some(first_letter) => {
                let upper: String = first_letter.to_uppercase().collect();
                vec_string.push(remove_accents(&upper) + chars.as_str())
            }
        }
    }
    vec_string.join(" ")
}

// ***********************
// * FONCTIONS PUBLIQUES *
// ***********************

/// Demande une saisie à l'utilisateur et la retourne après trim et lowercase.
///
/// Si `check` vaut `true`, une saisie vide est refusée et la question est
/// reposée. Si `check` vaut `false`, une saisie vide retourne `""`.
pub fn user_entry(question: &str, check: bool) -> String {
    loop {
        println!("{}", question);
        let response: String = read_input();
        match response.is_empty() {
            true => match check {
                true => println!("{}", EMPTY_FIELD),
                _ => return String::from(""),
            },
            false => return response.to_string(),
        }
    }
}

/// Demande une chaîne de caractères et la formate selon `upper_c`.
///
/// - `upper_c = true`  → majuscules intégrales (ex. : nom de famille)
/// - `upper_c = false` → Title Case désaccentué (ex. : titre de livre)
///
/// Le paramètre `check` est transmis à [`user_entry`] pour gérer
/// l'obligation de saisie.
pub fn ask_string(question: &str, check: bool, upper_c: bool) -> String {
    let response = user_entry(question, check);
    match upper_c {
        true => response.to_uppercase(),
        false => to_title_case_unicode(&response),
    }
}

/// Construit un [`Book`] en guidant l'utilisateur champ par champ.
///
/// Délègue la validation de chaque champ aux fonctions spécialisées :
/// [`ask_string`], [`ask_year`], [`ask_pages`].
pub fn build_book() -> Book {
    Book {
        author_name: ask_string("Nom de l'auteur : ", true, true),
        author_first_name: ask_string("Prénom de l'auteur : ", false, false),
        title: ask_string("Titre du livre : ", true, false),
        pub_year: ask_year("Année de publication : "),
        nb_pages: ask_pages("Nombre de pages : "),
    }
}
