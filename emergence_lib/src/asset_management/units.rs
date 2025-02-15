//! Asset loading for units

use crate::simulation::geometry::MapGeometry;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};

use super::{
    hexagonal_column,
    manifest::{Id, Unit},
    Loadable,
};

/// Stores material handles for the different tile types.
#[derive(Resource)]
pub(crate) struct UnitHandles {
    /// The scene for each type of structure
    pub(crate) scenes: HashMap<Id<Unit>, Handle<Scene>>,
    /// The raycasting mesh used to select units
    pub(crate) picking_mesh: Handle<Mesh>,
}

impl FromWorld for UnitHandles {
    fn from_world(world: &mut World) -> Self {
        /// The height of the picking box for a single unit.
        ///
        /// Hex tiles always have a diameter of 1.0.
        const PICKING_HEIGHT: f32 = 1.0;

        let map_geometry = world.resource::<MapGeometry>();
        let picking_mesh_object = hexagonal_column(&map_geometry.layout, PICKING_HEIGHT);
        let mut mesh_assets = world.resource_mut::<Assets<Mesh>>();
        let picking_mesh = mesh_assets.add(picking_mesh_object);

        let mut handles = UnitHandles {
            scenes: HashMap::default(),
            picking_mesh,
        };

        let asset_server = world.resource::<AssetServer>();

        // TODO: discover this from the file directory
        let unit_names = vec!["ant"];

        for str in unit_names {
            let structure_id = Id::from_string_id(str);
            let structure_path = format!("units/{str}.gltf#Scene0");
            let scene = asset_server.load(structure_path);
            handles.scenes.insert(structure_id, scene);
        }

        handles
    }
}

impl Loadable for UnitHandles {
    fn load_state(&self, asset_server: &AssetServer) -> LoadState {
        for (unit, scene_handle) in &self.scenes {
            let scene_load_state = asset_server.get_load_state(scene_handle);
            info!("{unit:?}'s scene is {scene_load_state:?}");

            if scene_load_state != LoadState::Loaded {
                return scene_load_state;
            }
        }

        LoadState::Loaded
    }
}
