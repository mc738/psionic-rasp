pub mod scene_loader;
pub mod scene_graph;

use crate::camera::Camera;
use crate::maths::Transform;
use glow::Context;
use crate::scenes::scene_graph::{SceneGraph, SceneGraphNode};

pub struct SceneInstance {
    pub graph: SceneGraph,
    pub transforms: TransformsCollection,
    pub main_camera: Camera,
}

/// All the transforms in a scene are kept in flat collection.
/// This is mainly so cycling through them is easy and quick.
/// It is easier to have a none nested/non-hiearchical structure with a hierarchical abstraction on top
/// than then other way round.
/// These are high level transforms rather than finder level ones like per mesh primitive.
/// This idea is this is meant to be used for quick look-ups, tests etc. for a scene,
/// rather than representing something like a model.
/// It does require keeping a list of "pointers" and look-ups for values, but these values should be pretty stable while a scene is loaded.
/// In the future if dynamic loading and unloading of models and resources becomes a thing this could become a pain.
/// However, anything that might need to spawn things at runtime might be better handled with object pooling.
/// That way they still exist here but just won't be queried until active again.
pub struct TransformsCollection {
    root: Transform,
    transforms: Vec<Transform>,
}

type RenderableId = u32;

impl SceneInstance {
    pub fn create(
        nodes: Vec<SceneGraphNode>,
        transforms: Vec<Transform>,
        world_root: Transform,
        main_camera: Camera,
    ) -> Self {
        Self {
            main_camera,
            graph: SceneGraph { nodes },
            transforms: TransformsCollection {
                root: world_root,
                transforms,
            },
        }
    }

    /// Create a blank scene.
    /// This differs from `.new()` because its intention is to always just create the minimum necessary scene to prevent crashes.
    /// It doesn't need to actually be able to do anything.
    pub fn blank() -> Self {
        Self {
            main_camera: Camera::create(800., 600.),
            graph: SceneGraph { nodes: vec![] },
            transforms: TransformsCollection::new(),
        }
    }

    /// Commits the screen for rendering.
    /// This will update all dirty transforms.
    pub fn commit(&mut self) {
        self.main_camera.update_basis();
    }

    /// The method currently does nothing.
    /// It exists in case in the future scene instances have some resources that need freeing.
    pub fn free(&self, _gl: &Context) {}

    //pub fn get_renderable_objects(&self) -> &Vec<RenderableId> {
    //    &self.renderable_objects
    //}

    //pub fn get_renderable_object(&self, internal_id: &u32) -> Option<&u32> {
    //    self.renderable_objects.get(*internal_id as usize)
    //}
}

impl TransformsCollection {
    pub fn new() -> Self {
        Self {
            root: Transform::default(),
            transforms: vec![],
        }
    }
}
