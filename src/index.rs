use std::collections::HashMap;

use crate::document::Document;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SearchIndex {
    pub terms: HashMap<String, Vec<usize>>,
    pub documents: HashMap<usize, String>,
}

impl SearchIndex {
    pub fn new() -> Self {
        SearchIndex {
            terms: HashMap::new(),
            documents: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, document: &Document) {
        self.documents.insert(
            document.id,
            document.path.to_string_lossy().to_string(),
        );

        for word in document.content.split(|c: char| !c.is_alphanumeric() && c != '_') {
    if word.is_empty() {
        continue;
    }

    let word = word.to_lowercase();

    self.terms
        .entry(word)
        .or_insert(Vec::new())
        .push(document.id);
}
    }

    pub fn search(&self, query: &str) -> Vec<(usize, usize)> {
        let mut scores: HashMap<usize, usize> = HashMap::new();

        for word in query.split_whitespace() {
            let word = word.to_lowercase();

            if let Some(document_ids) = self.terms.get(&word) {
                for &document_id in document_ids {
                    *scores.entry(document_id).or_insert(0) += 1;
                }
            }
        }

        let mut results: Vec<(usize, usize)> = scores.into_iter().collect();

        results.sort_by(|a, b| b.1.cmp(&a.1));

        results
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let data = serde_json::to_string(self).unwrap();
        std::fs::write(path, data)
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let index = serde_json::from_str(&data).unwrap();

        Ok(index)
    }
}