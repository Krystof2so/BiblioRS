//! Validation et contraintes sur les saisies

use chrono::{Datelike, Local}; // https://docs.rs/chrono/latest/chrono/
use lazy_static::lazy_static; // https://docs.rs/lazy_static/latest/lazy_static/

// **************
// * CONSTANTES *
// **************
/// Année d'invention de l'imprimerie de Gutenberg : borne inférieure acceptée.
const GUTENBERG_YEAR: i32 = 1450;

static VALIDATE_DATE: &str = "Veuillez saisir une année valide (exemple : 2020).";
static NO_NUMBER: &str = "Veuillez saisir un nombre (positif).";
static ERR_NB_PAGES: &str = "Le nombre de pages ne peut être supérieur à 9999.";

// `lazy_static!` permet d'initialiser ces valeurs une seule fois au premier accès,
// plutôt qu'à chaque appel de fonction. Nécessaire ici car `String::format` et
// `Local::now()` ne sont pas évaluables à la compilation (contrairement à `const`).
// cf. https://blog.logrocket.com/rust-lazy-static-pattern/
lazy_static! {
    static ref ACTUAL_YEAR: i32 = Local::now().year();
    static ref DATE_ERR: String = format!(
        "L'année doit être comprise entre {} et {}",
        GUTENBERG_YEAR, *ACTUAL_YEAR
    );
}

// *************
// * Fonctions *
// *************
/// Demande et valide une année de publication.
///
/// N'accepte que les entiers compris entre [`GUTENBERG_YEAR`] et l'année courante.
/// Boucle jusqu'à obtenir une saisie valide.
pub fn ask_year(question: &str) -> i32 {
    loop {
        let year = super::user_input::user_entry(question, true);
        match year.parse::<i32>() {
            Ok(year) if year >= GUTENBERG_YEAR && year <= *ACTUAL_YEAR => return year,
            Ok(_) => println!("{}", *DATE_ERR),
            Err(_) => println!("{}", VALIDATE_DATE),
        }
    }
}

/// Demande et valide un nombre de pages.
///
/// Retourne le nombre formaté sur 4 chiffres (`"0042"`), ce format étant
/// utilisé pour construire l'identifiant du livre (cf. `Book::id_book`).
/// Un champ vide est accepté et traité comme `"0000"`.
/// La valeur maximale acceptée est 9999.
pub fn ask_pages(question: &str) -> String {
    loop {
        let nb_pages = super::user_input::user_entry(question, false);
        match nb_pages.parse::<i32>() {
            Ok(nb_pages) => {
                if nb_pages <= 10000 {
                    return format!("{:04}", nb_pages);
                } else {
                    println!("{ERR_NB_PAGES}");
                }
            }
            Err(_) if nb_pages.is_empty() => return String::from("0000"),
            Err(_) => println!("{NO_NUMBER}"),
        }
    }
}
