//! Définition de la structure [`Book`] et de ses méthodes.

use crate::utils::remove_accents;

/// Représente un livre avec ses métadonnées bibliographiques.
///
/// Les champs sont publics pour permettre la construction directe
/// depuis [`crate::input::user_input::build_book`].
#[derive(Debug)]
pub struct Book {
    pub author_name: String,
    pub author_first_name: String,
    pub title: String,
    pub pub_year: i32,
    /// Stocké sur 4 chiffres avec zéros de remplissage (ex. `"0042"`),
    /// ce format étant utilisé dans la clé générée par [`Book::id_book`].
    pub nb_pages: String,
}

impl Book {
    /// Extrait et normalise les 4 premiers caractères d'une chaîne pour la clé.
    ///
    /// - Les accents sont supprimés via [`remove_accents`]
    /// - Les espaces et les caractères manquants sont remplacés par `'X'`
    /// - Le résultat est toujours en majuscules et fait exactement 4 caractères
    ///
    /// Exemple : `"île"` → `"ILXX"`, `"Le Monde"` → `"LEXM"`
    fn only_4_chars(my_string: &str) -> String {
        let mut four_chars: String = remove_accents(my_string).chars().take(4).collect();
        // Complète jusqu'à 4 caractères si la chaîne source est trop courte
        while four_chars.len() < 4 {
            four_chars.push('X');
        }
        four_chars.replace(" ", "X").to_uppercase()
    }

    /// Génère un identifiant unique pour le livre.
    ///
    /// Format : `<4 lettres auteur><année><4 lettres titre><pages>`  
    /// Exemple : `"HUGO1862MISE0823"`
    pub fn id_book(&self) -> String {
        format!(
            "{}{}{}{}",
            Self::only_4_chars(&self.author_name),
            self.pub_year,
            Self::only_4_chars(&self.title),
            self.nb_pages,
        )
    }

    /// Retourne une représentation lisible du livre.
    ///
    /// Le nombre de pages est affiché sans zéros de remplissage
    /// (`"0042"` → `"42"`), contrairement au format interne utilisé
    /// dans [`Book::id_book`].
    pub fn display(&self) -> String {
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
}
