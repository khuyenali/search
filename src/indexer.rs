mod database;
mod inverted_index;
mod parser;
mod tokenizer;

use inverted_index::InvertedIndex;
use tokenizer::Tokenizer;

use std::{
    collections::HashMap,
    io::{self, Write},
    path::Path,
};

use self::database::Database;

const INVERTED_INDEX_FILE: &str = "inverted_index.json";
const DATABASE_FILE: &str = "database.json";

pub enum Config<'a> {
    Build(&'a str),
    Load(&'a str),
}

pub struct Indexer {
    inverted_index: InvertedIndex,
    database: Database,
    tokenizer: Tokenizer,
}

impl Indexer {
    pub fn build(config: Config) -> Result<Self, io::Error> {
        println!("Start indexing");
        let mut indexer = Self {
            inverted_index: InvertedIndex::new(None),
            database: Database::new(),
            tokenizer: Tokenizer::new(),
        };

        match config {
            Config::Build(dir_path) => indexer.build_inverted_index(dir_path.to_string())?,
            Config::Load(file_path) => {
                indexer.database.import(DATABASE_FILE)?;
                indexer.load_inverted_index(file_path)?;
            }
        };

        println!("Build completed!");
        Ok(indexer)
    }

    pub fn run(&self) -> Result<(), io::Error> {
        loop {
            let mut buffer = String::new();
            print!("Query: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer)?;

            let document_ids = self.inverted_index.search(&buffer.trim(), &self.tokenizer);
            let docuemnt_names = self.database.get_names(&document_ids);
            for doc_name in docuemnt_names {
                println!("{doc_name}");
            }

            // break;
        }

        // Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let document_ids = self.inverted_index.search(query.trim(), &self.tokenizer);
        let docuemnt_names = self.database.get_names(&document_ids);

        docuemnt_names.collect()
    }

    fn build_inverted_index(&mut self, dir_path: String) -> io::Result<()> {
        // let tokenizer = Tokenizer::new();
        let root_path = Path::new(&dir_path);

        parser::parse_dir(root_path, &mut |file_name, text| {
            let document_id = self.database.add(file_name);

            let count_terms = count_terms(text, &self.tokenizer);
            self.inverted_index.add_document(document_id, count_terms);
        })?;

        // while let Some((file_name, text)) = parser.next() {
        // }

        self.inverted_index.calculate_rank(); // important!!!
        self.inverted_index.write_to_file(INVERTED_INDEX_FILE)?;
        self.database.export(DATABASE_FILE)?;

        Ok(())
    }

    fn load_inverted_index(&mut self, file_path: &str) -> io::Result<()> {
        self.inverted_index.read_from_file(file_path)?;
        Ok(())
    }
}

fn count_terms(text: String, tokenizer: &Tokenizer) -> HashMap<String, usize> {
    let mut count_terms: HashMap<String, usize> = HashMap::new();

    for line in text.lines() {
        for term in tokenizer.tokenizer_document(line) {
            match count_terms.get_mut(&term) {
                Some(c) => {
                    *c += 1;
                }
                None => {
                    count_terms.insert(term, 1);
                }
            };
        }
    }

    count_terms
}
