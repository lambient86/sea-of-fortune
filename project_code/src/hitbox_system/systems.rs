use bevy::prelude::*;
use bevy::math::Aabb2d;
use bevy::sprite::MaterialMesh2dBundle;
use crate::hitbox_system::components::{Hitbox, Hurtbox, Colliding};

// System to check collisions between hitboxes and hurtboxes
pub fn check_hitbox_hurtbox_collisions(
    hitbox_query: Query<(Entity, &Transform, &Hitbox)>,
    mut hurtbox_query: Query<(Entity, &Transform, &Hurtbox)>,
    mut commands: Commands,
) {
    // Iterate through all entities with hurtboxes
    for (hurtbox_entity, hurtbox_transform, hurtbox) in hurtbox_query.iter_mut() {
        let hurtbox_aabb = transform_aabb(hurtbox.aabb, hurtbox_transform);

        // Check against all hitboxes
        for (hitbox_entity, hitbox_transform, hitbox) in hitbox_query.iter() {
            // Avoid self-collision
            if hurtbox_entity != hitbox_entity {
                let hitbox_aabb = transform_aabb(hitbox.aabb, hitbox_transform);

                // Check for overlap
                if aabbs_overlap(hurtbox_aabb, hitbox_aabb) {
                    // Mark the entity with the hurtbox as colliding
                    commands.entity(hurtbox_entity).insert(Colliding);
                    break;
                }
            }
        }
    }
}

// System to draw debug visualizations for hitboxes and hurtboxes
pub fn draw_debug_boxes(
    mut gizmos: Gizmos,
    hitbox_query: Query<(&Transform, &Hitbox)>,
    hurtbox_query: Query<(&Transform, &Hurtbox)>,
) {
    // Draw white outlines for hitboxes
    for (transform, hitbox) in hitbox_query.iter() {
        draw_aabb(&mut gizmos, hitbox.aabb, transform, Color::WHITE);
    }

    // Draw red outlines for hurtboxes
    for (transform, hurtbox) in hurtbox_query.iter() {
        draw_aabb(&mut gizmos, hurtbox.aabb, transform, Color::RED);
    }
}

// Helper function to draw an AABB with the given color
fn draw_aabb(gizmos: &mut Gizmos, aabb: Aabb2d, transform: &Transform, color: Color) {
    let position = transform.translation.truncate();
    let scale = transform.scale.truncate();
    let min = aabb.min * scale + position;
    let max = aabb.max * scale + position;

    gizmos.rect_2d(
        (min + max) / 2.0,  // center point
        0.0,                // rotation (in radians)
        max - min,          // size
        color
    );
}

// Helper function to transform an AABB by a given transform
fn transform_aabb(aabb: Aabb2d, transform: &Transform) -> Aabb2d {
    let position = transform.translation.truncate();
    let scale = transform.scale.truncate();
    
    Aabb2d::new(
        aabb.min * scale + position,
        aabb.max * scale + position,
    )
}

// Helper function to check if two AABBs overlap
fn aabbs_overlap(a: Aabb2d, b: Aabb2d) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x && a.min.y < b.max.y && a.max.y > b.min.y
}

// Function to create a hitbox for an entity
pub fn create_hitbox(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec2,
) {
    let half_size = size / 2.0;
    let aabb = Aabb2d::new(offset - half_size, offset + half_size);
    commands.entity(entity).insert(Hitbox { aabb });
}

// Function to create a hurtbox for an entity
pub fn create_hurtbox(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec2,
) {
    let half_size = size / 2.0;
    let aabb = Aabb2d::new(offset - half_size, offset + half_size);
    commands.entity(entity).insert(Hurtbox { aabb });
}