use std::{sync::mpsc, thread::{JoinHandle, self}};
use anyhow::Result;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use tokio::{sync::oneshot, task};

/// Message type for internal channel, passing around texts and return value senders
type Message = (String, oneshot::Sender<EncodedText>);
/// Type of the text after the encoding
type EncodedText = Vec<f32>; 

/// Manages the Sentence Embedding Model into a single thread to allow async requests
#[derive(Debug, Clone)]
pub struct Model {
    sender: mpsc::SyncSender<Message>,
}

impl Model {

    /// Spawn a model on a separate thread and return an instance to interact with it
    pub fn spawn() -> (JoinHandle<Result<()>>, Model) {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || Self::runner(receiver));
        (handle, Model { sender })
    }

    /// The model runner itself
    fn runner(receiver: mpsc::Receiver<Message>) -> Result<()> {
        // Needs to be in sync runtime, async doesn't work
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()?;

        while let Ok((text, sender)) = receiver.recv() {
            let encoded = {
				let encoded = model.encode(&[text])?;
				encoded[0].to_owned()
			};
            sender.send(encoded).expect("sending results");
        }

        Ok(())
    }

    /// Make the runner predict a sample and return the result
    pub async fn encode(&self, text: String) -> Result<EncodedText> {
        let (sender, receiver) = oneshot::channel();
        task::block_in_place(|| self.sender.send((text, sender)))?;
        Ok(receiver.await?)
    }

}
