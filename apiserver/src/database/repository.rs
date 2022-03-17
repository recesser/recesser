use mongodb::bson;

#[derive(Clone)]
pub struct RepositoryStore {
    _collection: mongodb::Collection<mongodb::bson::Document>,
}

impl RepositoryStore {
    pub fn new(collection: mongodb::Collection<bson::Document>) -> Self {
        Self {
            _collection: collection,
        }
    }
}
