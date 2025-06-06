use bevy::reflect::GetTypeRegistration;
use bevy::reflect::Typed;

use crate::prelude::*;

/// Saves the start-of-frame value of another component.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Previous<C: Component + Clone>(pub C);

impl<C: Component + Clone + Typed + FromReflect + GetTypeRegistration> Configure for Previous<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(First, save_to_previous::<C>);
    }
}

fn save_to_previous<C: Component + Clone>(
    mut previous_query: Query<(&mut Previous<C>, &C), Changed<C>>,
) {
    for (mut previous, current) in &mut previous_query {
        previous.0 = current.clone();
    }
}
