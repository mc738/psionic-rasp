use std::collections::HashMap;
use uuid::Uuid;

/// A type used for mapping external ids to internal ones.
/// An external id will be globally unique (a uuid).
/// However, for in engine purposes u32's are used.
/// These often represent indexes.
#[derive(Clone)]
pub struct InternalIdMap {
    map: HashMap<Uuid, u32>,
}

impl InternalIdMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_internal_id(&self, id: &Uuid) -> Option<u32> {
        // Clone here because this value will often be cached.
        // Having it borrowed might be problematic, but definitely something to come back and check.
        self.map.get(id).cloned()
    }
    
    pub fn add(&mut self, id: &Uuid, internal_id: u32) {
        match &self.map.contains_key(&id) {
            true => {}
            false => {
                // This could also be a deref, rather than a clone.
                self.map.insert(id.clone(), internal_id);
            }
        }
    }
}