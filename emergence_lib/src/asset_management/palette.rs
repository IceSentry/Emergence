//! A central source of truth for the game and UI's color palettes.

use bevy::prelude::Color;

/// The alpha value used for selection/hovering/other UI overlay
const OVERLAY_ALPHA: f32 = 0.5;

/// The hue of selected objects
pub(crate) const SELECTION_HUE: f32 = 100.;
/// The saturation of selected objects
pub(crate) const SELECTION_SATURATION: f32 = 0.5;
/// The lightness of selected objects
pub(crate) const SELECTION_LIGHTNESS: f32 = 0.6;
/// The color used to tint selected objects.
pub(crate) const SELECTION_COLOR: Color = Color::hsla(
    SELECTION_HUE,
    SELECTION_SATURATION,
    SELECTION_LIGHTNESS,
    OVERLAY_ALPHA,
);

/// The hue used to indicate that an action is forbidden.
pub(crate) const FORBIDDEN_HUE: f32 = 0.;

/// The hue of selected objects
pub(crate) const HOVER_HUE: f32 = 55.;
/// The saturation of selected objects
pub(crate) const HOVER_SATURATION: f32 = 0.5;
/// The lightness of selected objects
pub(crate) const HOVER_LIGHTNESS: f32 = 0.6;

/// The color used to tint hovered objects.
pub(crate) const HOVER_COLOR: Color =
    Color::hsla(HOVER_HUE, HOVER_SATURATION, HOVER_LIGHTNESS, OVERLAY_ALPHA);

/// The hue value of ghost-like materials.
pub(crate) const GHOST_HUE: f32 = 0.0;
/// The saturation value of ghost-like materials.
pub(crate) const GHOST_SATURATION: f32 = 0.;
/// The lightness value of ghost-like materials.
pub(crate) const GHOST_LIGHTNESS: f32 = 0.9;
/// The alpha value of ghost-like materials.
pub(crate) const GHOST_ALPHA: f32 = 0.7;
/// The color used to tint ghosts
pub(crate) const GHOST_COLOR: Color =
    Color::hsla(GHOST_HUE, GHOST_SATURATION, GHOST_LIGHTNESS, GHOST_ALPHA);
/// The color used to tint selected ghosts
pub(crate) const SELECTED_GHOST_COLOR: Color = Color::hsla(
    SELECTION_HUE,
    SELECTION_SATURATION,
    SELECTION_LIGHTNESS,
    GHOST_ALPHA,
);

/// The color used to tint previews
pub(crate) const PREVIEW_COLOR: Color =
    Color::hsla(HOVER_HUE, HOVER_SATURATION, HOVER_LIGHTNESS, GHOST_ALPHA);
/// The color used to tint previews that cannot be built
pub(crate) const FORBIDDEN_PREVIEW_COLOR: Color = Color::hsla(
    FORBIDDEN_HUE,
    HOVER_SATURATION,
    HOVER_LIGHTNESS,
    GHOST_ALPHA,
);

/// The color used to tint objects that are both selected and hovered.
pub(crate) const SELECTION_AND_HOVER_COLOR: Color = Color::hsla(
    (SELECTION_HUE + HOVER_HUE) / 2.,
    (SELECTION_SATURATION + HOVER_SATURATION) / 2.,
    (SELECTION_LIGHTNESS + HOVER_LIGHTNESS) / 2.,
    OVERLAY_ALPHA,
);

/// The color used for columns of dirt underneath tiles
pub(crate) const COLUMN_COLOR: Color = Color::hsl(21., 0.6, 0.15);

/// The color of daylight
pub(crate) const LIGHT_SUN: Color = Color::Hsla {
    hue: 30.,
    saturation: 0.5,
    lightness: 1.,
    alpha: 1.,
};
