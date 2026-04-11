//! Fonctions utilitaires partagées entre les modules.

/// Lit une ligne depuis stdin et la retourne sans espaces superflus.
pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture ou de saisie utilisateur");
    input.trim().to_string()
}

/// Remplace les caractères accentués par leur équivalent ASCII, en conservant la casse.
/// Utilisée pour générer des identifiants normalisés (cf. `Book::id_book`).
pub fn remove_accents(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'À' | 'Â' | 'Ä' => 'A',
            'É' | 'È' | 'Ê' | 'Ë' => 'E',
            'Î' | 'Ï' => 'I',
            'Ô' | 'Ö' => 'O',
            'Ù' | 'Û' | 'Ü' => 'U',
            'Ç' => 'C',
            other => other,
        })
        .collect()
}
