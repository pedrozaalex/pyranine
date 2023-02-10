use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn highlight(code: &str, language: &str, theme: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes[theme];
    let syntax = syntax_set.find_syntax_by_token(language).unwrap();
    let html = highlighted_html_for_string(code, &syntax_set, syntax, theme);
    html.unwrap()
}
