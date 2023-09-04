use bevy::prelude::*;

use super::{
    style::{ComputedStyle, StyleAsset, UpdateComputedStyle},
    ViewElement,
};

/// A controller is an object which attaches to a UiComponent and handles events.
#[bevy_trait_query::queryable]
pub trait Controller {
    // TODO: This does nothing yet.
    fn attach(&self, _commands: &mut Commands, _entity: Entity, _view: &ViewElement) {}

    fn update_styles(
        &self,
        commands: &mut Commands,
        entity: Entity,
        view: &ViewElement,
        assets: &Assets<StyleAsset>,
    ) {
        let mut computed = ComputedStyle::default();
        self.compute_style(&mut computed, view, assets);
        commands.add(UpdateComputedStyle { entity, computed });
    }

    fn compute_style(
        &self,
        computed: &mut ComputedStyle,
        view: &ViewElement,
        assets: &Assets<StyleAsset>,
    ) {
        for handle in view.styleset_handles.iter() {
            if let Some(style) = assets.get(handle) {
                info!("Applying style.");
                style.apply_to(computed);
            } else {
                warn!("Failed to load style.");
            }
        }
        if let Some(ref inline) = view.inline_style {
            inline.apply_to(computed);
        }
    }
}

// pub enum UiEvent {
//     // Triggered by a pointer up event while active (not rolled off).
//     Click(PointerEvent),
//     // Wheel(PointerEvent),

//     // Fired continuously while the widget state is updating.
//     Input(ChangeEvent),

//     // Fired when widget has finished updating.
//     Change(ChangeEvent),

//     // Focus events.
//     Focus(FocusEvent),
//     Blur(FocusEvent),
// }
