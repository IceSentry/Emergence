//! Asset loading for structures

use crate::{
    asset_management::hexagonal_column, enum_iter::IterableEnum,
    player_interaction::selection::ObjectInteraction, simulation::geometry::MapGeometry,
    structures::construction::GhostKind,
};
use bevy::{asset::LoadState, prelude::*, utils::HashMap};

use super::{
    manifest::{Id, Structure},
    Loadable,
};

/// Stores material handles for the different tile types.
#[derive(Resource)]
pub(crate) struct StructureHandles {
    /// The scene for each type of structure
    pub(crate) scenes: HashMap<Id<Structure>, Handle<Scene>>,
    /// The materials used for tiles when they are selected or otherwise interacted with
    pub(crate) ghost_materials: HashMap<GhostKind, Handle<StandardMaterial>>,
    /// The raycasting mesh used to select structures
    pub(crate) picking_mesh: Handle<Mesh>,
}

impl FromWorld for StructureHandles {
    fn from_world(world: &mut World) -> Self {
        /// The height of the picking box for a single structure.
        ///
        /// Hex tiles always have a diameter of 1.0.
        const PICKING_HEIGHT: f32 = 1.0;

        let map_geometry = world.resource::<MapGeometry>();
        let picking_mesh_object = hexagonal_column(&map_geometry.layout, PICKING_HEIGHT);
        let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();
        let picking_mesh = mesh_assets.add(picking_mesh_object);

        let mut material_assets = world.resource_mut::<Assets<StandardMaterial>>();

        let mut interaction_materials = HashMap::new();
        for variant in ObjectInteraction::variants() {
            if let Some(material) = variant.material() {
                let material_handle = material_assets.add(material);
                interaction_materials.insert(variant, material_handle);
            }
        }

        let mut ghost_materials = HashMap::new();
        for variant in GhostKind::variants() {
            let material_handle = material_assets.add(variant.material());
            ghost_materials.insert(variant, material_handle);
        }

        let mut handles = StructureHandles {
            scenes: HashMap::default(),
            ghost_materials,
            picking_mesh,
        };

        let asset_server = world.resource::<AssetServer>();

        // TODO: discover this from the file directory
        let structure_names = vec!["acacia", "leuco", "ant_hive", "hatchery"];

        for id in structure_names {
            let structure_id = Id::from_string_id(id);
            let structure_path = format!("structures/{id}.gltf#Scene0");
            let scene = asset_server.load(structure_path);
            handles.scenes.insert(structure_id, scene);
        }

        handles
    }
}

impl Loadable for StructureHandles {
    fn load_state(&self, asset_server: &AssetServer) -> LoadState {
        for (structure, scene_handle) in &self.scenes {
            let scene_load_state = asset_server.get_load_state(scene_handle);
            info!("{structure:?}'s scene is {scene_load_state:?}");

            if scene_load_state != LoadState::Loaded {
                return scene_load_state;
            }
        }

        LoadState::Loaded
    }
}
