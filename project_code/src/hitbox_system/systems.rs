use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;
use crate::hitbox_system::components::{Hitbox, Hurtbox, Colliding};

pub fn check_hitbox_collisions(
    hitbox_query: Query<(Entity, &Transform, &Hitbox)>,
    mut commands: Commands,
) {
    let mut colliding_entities = Vec::new();

    for (entity_a, transform_a, hitbox_a) in hitbox_query.iter() {
        for (entity_b, transform_b, hitbox_b) in hitbox_query.iter() {
            if entity_a != entity_b {
                let aabb_a = transform_aabb(hitbox_a.aabb, transform_a);
                let aabb_b = transform_aabb(hitbox_b.aabb, transform_b);

                if aabbs_overlap(aabb_a, aabb_b) {
                    colliding_entities.push(entity_a);
                    colliding_entities.push(entity_b);
                }
            }
        }
    }

    for entity in colliding_entities {
        commands.entity(entity).insert(Colliding);
    }
}

pub fn check_hurtbox_collisions(
    hitbox_query: Query<(Entity, &Transform, &Hitbox)>,
    mut hurtbox_query: Query<(Entity, &Transform, &Hurtbox)>,
    mut commands: Commands,
) {
    for (hurtbox_entity, hurtbox_transform, hurtbox) in hurtbox_query.iter_mut() {
        let hurtbox_aabb = transform_aabb(hurtbox.aabb, hurtbox_transform);

        for (hitbox_entity, hitbox_transform, hitbox) in hitbox_query.iter() {
            if hurtbox_entity != hitbox_entity {
                let hitbox_aabb = transform_aabb(hitbox.aabb, hitbox_transform);

                if aabbs_overlap(hurtbox_aabb, hitbox_aabb) {
                    commands.entity(hurtbox_entity).insert(Colliding);
                    break;
                }
            }
        }
    }
}

fn transform_aabb(aabb: Aabb2d, transform: &Transform) -> Aabb2d {
    let position = transform.translation.truncate();
    let scale = transform.scale.truncate();
    
    Aabb2d::new(
        aabb.min * scale + position,
        aabb.max * scale + position,
    )
}

fn aabbs_overlap(a: Aabb2d, b: Aabb2d) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x && a.min.y < b.max.y && a.max.y > b.min.y
}

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