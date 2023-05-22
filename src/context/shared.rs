use crate::models::SupportCollection;

pub struct CollectionContainer {
    #[allow(dead_code)]
    pub support: SupportCollection,
}
impl CollectionContainer {
    pub fn new(support: SupportCollection) -> CollectionContainer {
        CollectionContainer { support }
    }
}

pub struct AppState {
    #[allow(dead_code)]
    pub container: CollectionContainer,
}
