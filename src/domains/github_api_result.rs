use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ContainerMetadata {
    pub container: Container,
}

#[derive(Deserialize, Debug)]
pub struct ContainerImage {
    pub id: usize,
    pub name: String,
    pub metadata: ContainerMetadata,
    pub created_at: String,
    pub updated_at: String,
}
