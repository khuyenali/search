use std::collections::HashSet;

pub struct Tokenizer {
    stop_words: HashSet<String>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let words = stop_words::get(stop_words::LANGUAGE::English);
        Self {
            stop_words: HashSet::from_iter(words.into_iter()),
        }
    }

    pub fn tokenizer_document<'a>(&'a self, line: &'a str) -> impl Iterator<Item = String> + 'a {
        line.trim()
            .split_whitespace()
            .map(|s| s.trim_start_matches([',', '.', '[', '(', ':', '\n']))
            .map(|s| s.trim_end_matches([',', '.', ']', ')', ':', ';', '\n']))
            .filter(|s| s.len() > 2)
            .map(|s| s.to_lowercase())
            .filter(|s| self.stop_words.get(s).is_none())
    }

    pub fn tokenizer_query<'a>(&'a self, term: &'a str) -> Option<String> {
        let mut term = term.trim_start_matches([',', '.', '[', '(', ':', '\n']);
        term = term.trim_end_matches([',', '.', ']', ')', ':', ';', '\n']);

        if term.len() < 3 {
            return None;
        }

        let term = term.to_lowercase();
        if let None = self.stop_words.get(&term) {
            Some(term)
        } else {
            None
        }
    }
}
