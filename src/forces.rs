use bevy::prelude::*;
use bevy_rapier2d::{dynamics::ExternalForce, plugin::PhysicsSet};

/// Allows forces to be added together each frame without accumulating
/// forces from previous frames.
///
/// **Usage:**
/// ```
/// fn main() {
///     App::new()
///         .add_plugins(AddForcesEachFrame)
///         .add_systems(Update, affect_force.in_set(UpdateForce))
/// }
///
/// // Must run every frame to continue effecting the entity (as simulated by bevy_rapier2d)
/// fn affect_force(mut forces: Query<&mut ExternalForce>) {
///    for mut force in forces.iter_mut() {
///       force.force += Vec2::new(0.0, 1.0);
///   }
/// }
/// ```
pub struct AddForcesEachFrame;

impl Plugin for AddForcesEachFrame {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UpdateForce.before(PhysicsSet::SyncBackend))
            .add_systems(
                Update,
                zero_forces
                    .before(UpdateForce)
                    .before(PhysicsSet::SyncBackend),
            );
    }
}

/// SystemSet label for the systems that update the ExternalForce component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct UpdateForce;

fn zero_forces(mut bodies: Query<&mut ExternalForce>) {
    for mut ext_force in bodies.iter_mut() {
        ext_force.force = Vec2::ZERO;
        ext_force.torque = 0.0;
    }
}
