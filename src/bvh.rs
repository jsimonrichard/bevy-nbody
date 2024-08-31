use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::{ExternalForce, ReadMassProperties};

use crate::{aabb::Aabb, forces::UpdateForce};

/// This plugin simulates the force gravity originating from entities with the
/// `LargeMass` component. To optimize the force calculation, it maintains a binary
/// BVH tree of the entities with the `LargeMass` component.
pub struct SimulateGravity;

impl Plugin for SimulateGravity {
    fn build(&self, app: &mut App) {
        app.init_resource::<BvhConfig>()
            .insert_resource(BvhConfig::default())
            .insert_resource(BvhTree::default())
            .add_systems(Update, handle_new_entities.before(apply_gravity))
            .add_systems(Update, apply_gravity.in_set(UpdateForce));
    }
}

#[derive(Resource)]
pub struct BvhConfig {
    pub g: f32,
    pub theta: f32,
    pub bin_count: usize,
}

impl Default for BvhConfig {
    fn default() -> Self {
        Self {
            g: 1.0,
            theta: 0.5,
            bin_count: 8,
        }
    }
}

#[derive(Component)]
pub struct LargeMass;

#[derive(Resource, Debug, Default)]
struct BvhTree {
    /// Root of the binary BVH tree
    root: Option<BvhNode>,

    /// Map from entity to node data in the tree used to update the tree when entities move
    entity_table: HashMap<Entity, BvhData>,
}

impl BvhTree {
    fn add_node(&mut self, node: BvhNode) {
        if let Some(root) = &mut self.root {
            root.add_node(node);
        } else {
            self.root = Some(node);
        }
    }
}

#[derive(Debug)]
struct BvhNode {
    data: BvhData,
    contents: BvhContents,
}

impl BvhNode {
    fn add_node(&mut self, node: BvhNode) {
        match &mut self.contents {
            BvhContents::Leaf { .. } => {
                let new_self = BvhNode {
                    data: self.data.union(&node.data),
                    // Placeholder entity
                    contents: BvhContents::Leaf {
                        entity: Entity::from_raw(0),
                    },
                };

                let left = std::mem::replace(self, new_self);

                self.contents = BvhContents::Branch {
                    left: Box::new(left),
                    right: Box::new(node),
                };
            }
            BvhContents::Branch { left, right } => {
                let left_cost = left.data.bounds.union(&node.data.bounds).area();
                let right_cost = right.data.bounds.union(&node.data.bounds).area();

                if left_cost < right_cost {
                    left.add_node(node);
                } else {
                    right.add_node(node);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct BvhData {
    bounds: Aabb,
    mass: f32,
    center_of_mass: Vec2,
}

impl BvhData {
    fn union(&self, other: &Self) -> Self {
        Self {
            bounds: self.bounds.union(&other.bounds),
            mass: self.mass + other.mass,
            center_of_mass: (self.center_of_mass * self.mass + other.center_of_mass * other.mass)
                / (self.mass + other.mass),
        }
    }
}

#[derive(Debug)]
enum BvhContents {
    Leaf {
        entity: Entity,
    },
    Branch {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

fn handle_new_entities(
    mut bvh_tree: ResMut<BvhTree>,
    bodies: Query<(Entity, &Transform, &ReadMassProperties), Added<LargeMass>>,
) {
    for (entity, transform, mass_properties) in bodies.iter() {
        println!("{:?}.mass: {}", entity, mass_properties.mass);
        let point_2d = Vec2::new(transform.translation.x, transform.translation.y);
        let bounds = Aabb::new(point_2d, point_2d);
        let contents = BvhContents::Leaf { entity };
        let node = BvhNode {
            data: BvhData {
                bounds,
                mass: mass_properties.mass,
                center_of_mass: point_2d + mass_properties.local_center_of_mass,
            },
            contents,
        };

        bvh_tree.add_node(node);

        println!("{:?}", bvh_tree);
    }
}

// fn update_bvh(
//     mut bvh_tree: ResMut<BvhTree>,
//     bodies: Query<(Entity, &Transform, &ReadMassProperties)>,
// ) {
//     let mut nodes = vec![];

//     for (entity, transform, mass_properties) in bodies.iter() {
//         let bounds = Aabb::new(transform.translation, transform.translation);
//         let contents = BvhContents::Leaf {
//             mass: mass_properties.mass,
//         };
//         let node = BvhNode { bounds, contents };
//         nodes.push(node);
//     }

//     while nodes.len() > 1 {
//         let mut new_nodes = vec![];

//         for chunk in nodes.chunks(2) {
//             let left = chunk[0];
//             let right = chunk[1];
//             let bounds = left.bounds.union(&right.bounds);
//             let contents = BvhContents::Branch {
//                 left: Box::new(left),
//                 right: Box::new(right),
//             };
//             let node = BvhNode { bounds, contents };
//             new_nodes.push(node);
//         }

//         nodes = new_nodes;
//     }

//     bvh_tree.root = nodes.pop().unwrap();
// }

fn apply_gravity(
    mut bodies: Query<(Entity, &Transform, &ReadMassProperties, &mut ExternalForce)>,
    bvh_config: Res<BvhConfig>,
    bvh_tree: ResMut<BvhTree>,
) {
}

// fn simulate_gravity(mut bodies: Query<(&Transform, &ReadMassProperties, &mut ExternalForce)>) {
//     // TODO: calculate/update the BVHs

//     // Toy example
//     for (transform, mass_properties, mut ext_force) in bodies.iter_mut() {
//         let gravity = Vec2::new(0.0, -9.81 * mass_properties.mass);
//         let force = gravity * mass_properties.mass;
//         // ext_force.force = force;
//     }
// }
