//! Gestion de la saisie utilisateur.
//!
//! - [`check_input`] : validation et contraintes métier (année, nombre de pages)
//! - [`user_input`]  : lecture, formatage des entrées et construction d'un [`super::models::book::Book`]

pub mod check_input;
pub mod user_input;
