//! Syntax highlighting of the course languages (see `nrzlearn_model::highlight`).

use crate::kurs::Kurs;

pub type Token = nrzlearn_model::highlight::Token;

pub fn tokenize_in(src: &str, kurs: Kurs) -> Vec<Token> {
    nrzlearn_model::highlight::tokenize(src, kurs.id())
}
