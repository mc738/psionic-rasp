use crate::rendering::traits::Renderable;
use std::collections::HashMap;
use uuid::Uuid;

/// A type used for mapping external ids to internal ones.
/// An external id will be globally unique (a uuid).
/// However, for in engine purposes u32's are used.
/// These often represent indexes.
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
}

pub mod scene_manager;

pub struct SceneInstance<'a> {
    renderable_objects: SceneRenderableCollection<'a>,
}

pub struct SceneRenderableCollection<'a> {
    id_map: InternalIdMap,
    items: Vec<Box<&'a dyn Renderable>>,
}

impl<'a> SceneInstance<'a> {
    pub fn create() -> Self {
        Self {
            renderable_objects: SceneRenderableCollection::new(),
        }
    }

    pub fn get_renderable_objects(&self) -> &Vec<Box<&dyn Renderable>> {
        &self.renderable_objects.items
    }

    pub fn get_renderable_object(&self, internal_id: &u32) -> Option<&Box<&dyn Renderable>> {
        self.renderable_objects.get_item(internal_id)
    }
}

impl<'a> SceneRenderableCollection<'a> {
    pub fn new() -> Self {
        Self {
            id_map: InternalIdMap::new(),
            items: Vec::<Box<&dyn Renderable>>::new(),
        }
    }

    pub fn get_internal_id(&self, id: &Uuid) -> Option<u32> {
        self.id_map.get_internal_id(id)
    }

    /// Add an item and return it's internal id.
    /// If the item is all ready add, the id will simply be returned.
    ///
    /// # Arguments
    ///
    /// * `id`:
    /// * `item`:
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_item(&mut self, id: Uuid, item: &'a mut dyn Renderable) -> u32 {
        match self.id_map.get_internal_id(&id) {
            Some(r) => r,
            None => {
                let l = self.items.len();
                let index = l as u32;
                item.set_internal_id(index);
                self.items.push(Box::new(item));
                self.id_map.map.insert(id, index);
                index
            }
        }
    }

    pub fn remove_item(&mut self, id: &Uuid) -> Option<Box<&dyn Renderable>> {
        match self.id_map.get_internal_id(&id) {
            None => None,
            Some(r) => {
                self.id_map.map.remove(&id);
                Some(self.items.remove(r as usize))
            }
        }
    }

    pub fn get_item(&self, id: &u32) -> Option<&Box<&dyn Renderable>> {
        match self.items.len() > *id as usize {
            false => None,
            true => Some(self.items.get(*id as usize).unwrap()),
        }
    }
}
