use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io};

use super::tokenizer::Tokenizer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfIdf {
    pub document_id: usize,
    pub tf_idf: f64,
}

type Posting = Vec<TfIdf>;
type Term = String;

pub struct InvertedIndex {
    posting_list: HashMap<Term, Posting>,
    total_documents: usize,

    results_size: usize,
}

impl InvertedIndex {
    pub fn new(reults_size: Option<usize>) -> Self {
        Self {
            posting_list: HashMap::new(),
            total_documents: 0,

            results_size: reults_size.unwrap_or(10),
        }
    }

    pub fn add_document(&mut self, document_id: usize, count_terms: HashMap<Term, usize>) {
        if count_terms.len() == 0 {
            return;
        }

        self.total_documents += 1;
        for (term, term_count) in count_terms.into_iter() {
            let tf_idf = (1.0 + term_count as f64).log10();

            let new_tfidf = TfIdf {
                document_id,
                tf_idf,
            };

            match self.posting_list.get_mut(&term) {
                Some(posting) => {
                    posting.push(new_tfidf);
                }
                None => {
                    self.posting_list.insert(term, vec![new_tfidf]);
                }
            };
        }
    }

    pub fn calculate_rank(&mut self) {
        for (_, postings) in &mut self.posting_list {
            let idf = self.total_documents as f64 / postings.len() as f64;

            for posting in postings.iter_mut() {
                posting.tf_idf = (1.0 + posting.tf_idf.log10()) * idf.log10();
            }

            postings.sort_by(|a, b| b.tf_idf.partial_cmp(&a.tf_idf).unwrap());
        }
    }

    pub fn search(&self, query: &str, tokenizer: &Tokenizer) -> Vec<usize> {
        let mut results = vec![];
        let mut result_len = 0;

        for term in query.split_whitespace() {
            println!("before: {}", term);
            let term = tokenizer.tokenizer_query(term);
            if let None = term {
                continue;
            } 
            let term = term.unwrap();

            let mut tops = &Vec::new()[..];
            tops = if let Some(posting) = self.posting_list.get(&term) {
                let total_results = posting.len();
                if total_results > self.results_size {
                    // posting[0..10].iter().map(|p| p.document_id).collect()
                    &posting[0..self.results_size]
                } else {
                    &posting[..]
                }
            } else {
                tops
            };

            (result_len, results) = self.merge_results(result_len, results, tops);
        }

        results.truncate(result_len);
        results.iter().map(|result| result.document_id).collect()
    }

    fn merge_results(
        &self,
        result_len: usize,
        mut results: Vec<TfIdf>,
        tops: &[TfIdf],
    ) -> (usize, Vec<TfIdf>) {
        if results.len() == 0 && result_len == 0 {
            return (tops.len(), tops.iter().map(|tfidf| tfidf.clone()).collect());
        } else if results.len() > 0 && result_len == 0 {
            return (0, results);
        }

        // for tfidf in results.iter() {
        //     results[0] = tfidf.clone();
        // };

        let mut index = 0;

        for i in 0..result_len {
            let same = tops
                .iter()
                .find(|top| top.document_id == results[i].document_id);
            if let Some(same) = same {
                results[index].document_id = results[i].document_id;
                results[index].tf_idf = results[i].tf_idf + same.tf_idf;
                index += 1;
            } else {
                results[i].tf_idf = 0.0
            }
        }

        // let mut tmp: HashMap<DocID, Index> = HashMap::new();
        //
        // for (i, tfidf) in results.iter().enumerate() {
        //     tmp.insert(tfidf.document_id, i);
        // }
        //
        // let mut last_index = self.results_size;
        //
        // for tfidf in tops {
        //     match tmp.get(&tfidf.document_id) {
        //         Some(index) => results[*index].tf_idf += tfidf.tf_idf,
        //         None => {
        //             if last_index < results.len() {
        //                 results[last_index] = tfidf.clone();
        //             } else {
        //                 results.push(tfidf.clone());
        //             }
        //
        //             last_index += 1;
        //         }
        //     }
        // }

        results.sort_by(|a, b| b.tf_idf.partial_cmp(&a.tf_idf).unwrap());

        (index, results)
    }

    pub fn write_to_file(&self, file_path: &str) -> Result<(), io::Error> {
        let writer = File::create(file_path)?;
        serde_json::to_writer(writer, &self.posting_list)?;
        Ok(())
    }

    pub fn read_from_file(&mut self, file_path: &str) -> Result<(), io::Error> {
        let reader = File::open(file_path)?;
        self.posting_list = serde_json::from_reader(reader)?;
        Ok(())
    }
}
