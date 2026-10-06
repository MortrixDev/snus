use crate::tokenize::tokenize;
use crate::Document;

use std::collections::HashMap;
use std::path::PathBuf;

const K1: f32 = 1.2;
const B: f32 = 0.75;
const TITLE_WEIGHT: f32 = 4.0;
const HEADINGS_WEIGHT: f32 = 2.0;

pub type DocId = u32;

pub struct DocInfo {
    pub path: PathBuf,
    pub title: Option<String>,
}

pub struct Occurrence {
    doc: DocId,
    positions: Vec<u32>,
}

#[derive(Default)]
struct Field {
    occurrences: HashMap<String, Vec<Occurrence>>,
    lens: Vec<u32>,
    total_len: u64,
}

impl Field {
    fn add(&mut self, id: DocId, terms: Vec<String>) {
        let len = terms.len() as u32;
        self.lens.push(len);
        self.total_len += len as u64;

        let mut positions: HashMap<String, Vec<u32>> = HashMap::new();
        for (pos, term) in terms.into_iter().enumerate() {
            positions.entry(term).or_default().push(pos as u32);
        }

        for (term, positions) in positions {
            self.occurrences
                .entry(term)
                .or_default()
                .push(Occurrence { doc: id, positions });
        }
    }

    fn score(&self, term: &str, weight: f32, scores: &mut HashMap<DocId, f32>) {
        let Some(occurrences) = self.occurrences.get(term) else {
            return;
        };

        let n = self.lens.len() as f32;
        let avgdl = (self.total_len as f32 / n).max(1.0);
        let df = occurrences.len() as f32;
        let idf = ((n - df + 0.5) / (df + 0.5) + 1.0).ln();

        for occ in occurrences {
            let tf = occ.positions.len() as f32;
            let dl = self.lens[occ.doc as usize] as f32;
            let norm = tf + K1 * (1.0 - B + B * dl / avgdl);
            *scores.entry(occ.doc).or_default() += weight * idf * tf * (K1 + 1.0) / norm;
        }
    }
}

#[derive(Default)]
pub struct Index {
    pub docs: Vec<DocInfo>,
    body: Field,
    title: Field,
    headings: Field,
}

impl Index {
    pub fn add(&mut self, doc: Document) {
        let id = self.docs.len() as DocId;

        self.body.add(id, tokenize(&doc.body));
        self.title
            .add(id, tokenize(doc.title.as_deref().unwrap_or("")));
        self.headings.add(id, tokenize(&doc.headings.join(" ")));

        self.docs.push(DocInfo {
            path: doc.path,
            title: doc.title,
        });
    }

    pub fn search(&self, query: &str) -> Vec<(DocId, f32)> {
        let mut terms = tokenize(query);
        terms.sort();
        terms.dedup();

        let mut scores: HashMap<DocId, f32> = HashMap::new();
        for term in &terms {
            self.body.score(term, 1.0, &mut scores);
            self.title.score(term, TITLE_WEIGHT, &mut scores);
            self.headings.score(term, HEADINGS_WEIGHT, &mut scores);
        }

        let mut results: Vec<(DocId, f32)> = scores.into_iter().collect();
        results.sort_by(|a, b| b.1.total_cmp(&a.1));
        results
    }
}
