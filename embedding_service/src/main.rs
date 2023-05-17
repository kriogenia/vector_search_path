use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Books {
    pub books: Vec<Book>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub summary: String,
}

impl Book {
    fn to_embedded(self, embeddings: [f32; 384]) -> EmbeddedBook {
        EmbeddedBook {
            title: Some(self.title),
            author: Some(self.author),
            summary: Some(self.summary),
            embeddings,
        }
    }
}

#[derive(Debug)]
pub struct EmbeddedBook {
    pub title: Option<String>,

    pub author: Option<String>,

    pub summary: Option<String>,

    pub embeddings: [f32; 384],
}

fn main() -> anyhow::Result<()> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model()?;

    let json = fs::read_to_string("data/books.json")?;
    let library: Books = serde_json::from_str(&json)?;

    for book in library.books.clone() {
        let embeddings = model.encode(&[book.clone().summary])?;
		let embedded = book.to_embedded(to_array(embeddings[0].as_slice()));
		println!("{:?} : {:?}", embedded.title, embedded.embeddings)
    }

    Ok(())
}

fn to_array(barry: &[f32]) -> [f32; 384] {
    barry.try_into().expect("slice with incorrect length")
}