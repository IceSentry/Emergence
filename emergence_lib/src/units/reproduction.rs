//! Making more units

use bevy::prelude::*;
use rand::prelude::IteratorRandom;
use rand::thread_rng;

use crate::{
    asset_management::{
        manifest::{Id, UnitManifest},
        units::UnitHandles,
    },
    simulation::geometry::{MapGeometry, TilePos},
    structures::crafting::{ActiveRecipe, CraftingState},
};

use super::UnitBundle;

/// Spawn ants when eggs have hatched
pub(super) fn hatch_ant_eggs(
    structure_query: Query<(&TilePos, &CraftingState, &ActiveRecipe)>,
    map_geometry: Res<MapGeometry>,
    unit_handles: Res<UnitHandles>,
    unit_manifest: Res<UnitManifest>,
    mut commands: Commands,
) {
    let rng = &mut thread_rng();

    // PERF: I don't like the linear time polling here. This really feels like it should be push-based with one-shot system callbacks on the recipe.
    for (tile_pos, crafting_state, active_recipe) in structure_query.iter() {
        if let Some(recipe_id) = active_recipe.recipe_id() {
            if *recipe_id == Id::hatch_ants()
                && matches!(crafting_state, CraftingState::RecipeComplete)
            {
                let empty_neighbors = tile_pos.empty_neighbors(&map_geometry);
                if let Some(pos_to_spawn) = empty_neighbors.into_iter().choose(rng) {
                    // TODO: use a unit manifest instead
                    commands.spawn(UnitBundle::new(
                        Id::ant(),
                        pos_to_spawn,
                        unit_manifest.get(Id::from_string_id("ant")).clone(),
                        &unit_handles,
                        &map_geometry,
                    ));
                }
            }
        }
    }
}
