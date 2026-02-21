// Fonctions de validation

use chrono::{Datelike, Local}; // https://docs.rs/chrono/latest/chrono/
use lazy_static::lazy_static; // https://docs.rs/lazy_static/latest/lazy_static/

use crate::input::user_input::user_entry;

// ***********************************
// * DONNEES STATIQUES ET CONSTANTES *
// ***********************************
lazy_static! {
    // Unique évaluation, et non plus à chaque appel de la fonction
    // cf. https://blog.logrocket.com/rust-lazy-static-pattern/
    static ref ACTUAL_YEAR: i32 = Local::now().year();
    static ref DATE_ERR: String = format!(
        "L'année doit être comprise entre {} et {}",
        GUTENBERG_YEAR, *ACTUAL_YEAR
    );
}

const GUTENBERG_YEAR: i32 = 1450;
static VALIDATE_DATE: &str = "Veuillez saisir une année valide (exemple : 2020).";
static NO_NUMBER: &str = "Veuillez saisir un nombre (positif).";
static ERR_NB_PAGES: &str = "Le nombre de pages ne peut être supérieur à 9999.";

// *************
// * Fonctions *
// *************
pub fn ask_year(question: &str) -> i32 {
    // validation de saisie d'une année de publication
    loop {
        let year = user_entry(question, true);
        // On tente de convertir la saisie en i32
        match year.parse::<i32>() {
            // Gestion par match plus concise (tous les cas de saisie prévus)
            // Seule sortie de boucle et de la fonction :
            Ok(year) if year >= GUTENBERG_YEAR && year <= *ACTUAL_YEAR => return year,
            Ok(_) => println!("{}", *DATE_ERR),
            Err(_) => println!("{}", VALIDATE_DATE),
        }
    }
}

pub fn ask_pages(question: &str) -> String {
    // validation nombre de pages
    loop {
        let nb_pages = user_entry(question, false);
        match nb_pages.parse::<u32>() {
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
