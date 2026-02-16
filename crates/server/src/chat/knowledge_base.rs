use anyhow::Result;
use rig::embeddings::EmbeddingsBuilder;
use rig::providers::openai::EmbeddingModel;
use rig::vector_store::in_memory_store::{InMemoryVectorIndex, InMemoryVectorStore};
use std::path::Path;
use tokio::fs;
use tracing::info;

#[derive(Clone)]
pub struct KnowledgeBase {
    pub store: InMemoryVectorStore<String>,
    pub embedding_model: EmbeddingModel,
}

impl KnowledgeBase {
    pub async fn new<P: AsRef<Path>>(path: P, embedding_model: EmbeddingModel) -> Result<Self> {
        let mut documents_dir = fs::read_dir(path).await?;
        let mut documents = vec![];

        while let Some(entry) = documents_dir.next_entry().await? {
            if entry.file_type().await?.is_file() {
                let content = fs::read_to_string(entry.path()).await?;
                info!("content:{content}");
                documents.push(content);
            }
        }

        let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
            .documents(documents)?
            .build()
            .await?;

        let store = InMemoryVectorStore::from_documents(embeddings);

        Ok(Self {
            store,
            embedding_model,
        })
    }

    pub fn index(&self) -> InMemoryVectorIndex<EmbeddingModel, String> {
        self.store.clone().index(self.embedding_model.clone())
    }
}
