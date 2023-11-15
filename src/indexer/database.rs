use std::{fs::File, io};

pub struct Database {
    documents: Vec<String>,
    current_id: usize,
}

impl Database {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            current_id: 0,
        }
    }

    pub fn add(&mut self, document: String) -> usize {
        self.documents.push(document);
        self.current_id += 1;

        self.current_id - 1
    }

    // pub fn get_name(&self, document_id: usize) -> Option<&String> {
    //     self.documents.get(document_id)
    // }

    pub fn get_names<'a>(
        &'a self,
        document_ids: &'a [usize],
    ) -> impl Iterator<Item = String> + 'a {
        document_ids.iter().map(|id| self.documents[*id].clone())
    }

    pub fn export(&self, file_path: &str) -> Result<(), io::Error> {
        let writer = File::create(file_path)?;
        serde_json::to_writer(writer, &self.documents)?;
        Ok(())
    }

    pub fn import(&mut self, file_path: &str) -> Result<(), io::Error> {
        let reader = File::open(file_path)?;
        self.documents = serde_json::from_reader(reader)?;
        Ok(())
    }
}
