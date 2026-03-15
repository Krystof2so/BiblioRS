// Contient les fonctions de saisies de l'utilisateur
use heck::ToTitleCase; // https://docs.rs/heck/latest/heck/
use std::io; // https://doc.rust-lang.org/std/

// **************
// * CONSTANTES *
// **************
// Les constantes de type chaîne de caractères doivent être déclarées comme statiques
// cf. https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
static EXPECT_ERR: &str = "Erreur lors de la lecture de la saisie utilisateur";
static EMPTY_FIELD: &str = "Ce champ ne peut être vide";

// ************
// * FONCIONS *
// ************
pub fn user_entry(question: &str, check: bool) -> String {
    // Pour demander une saisie utilisateur
    loop {
        println!("{}", question);
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect(EXPECT_ERR);
        let response = response.trim();
        match response.is_empty() {
            true => match check {
                true => println!("{}", EMPTY_FIELD),
                _ => return String::from(""),
            },
            false => return response.to_string(),
        }
    }
}

pub fn ask_string(question: &str, check: bool, upper_c: bool) -> String {
    // Pour demander une chaîne de caractères
    // privilégier la référence comme paramètre de fonction (plus performant)
    let response = user_entry(question, check);
    match upper_c {
        true => response.to_uppercase(),
        false => response.to_title_case(),
    }
}
