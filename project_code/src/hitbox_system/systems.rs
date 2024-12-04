use crate::hitbox_system::components::*;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

// System to check collisions between hitboxes and hurtboxes
pub fn check_hitbox_hurtbox_collisions(
    time: Res<Time>,
    hitbox_query: Query<(Entity, &Transform, &Hitbox)>,
    mut hurtbox_query: Query<(&Transform, &mut Hurtbox)>,
    mut commands: Commands,
) {
    // // Iterate through all entities with hurtboxes
    for (hurtbox_transform, mut hurtbox) in hurtbox_query.iter_mut() {
        if !hurtbox.iframe.finished() {
            hurtbox.iframe.tick(time.delta());
            //println!("Timer left: {}", hurtbox.iframe.remaining_secs());

            continue;
        }

        let hurtbox_pos = hurtbox_transform.translation.truncate() + hurtbox.offset;
        let hurtbox_min = hurtbox_pos - hurtbox.size / 2.0;
        let hurtbox_max = hurtbox_pos + hurtbox.size / 2.0;

        for (hitbox_entity, hitbox_transform, hitbox) in hitbox_query.iter() {
            if hurtbox.enemy != hitbox.enemy {
                let hitbox_pos = hitbox_transform.translation.truncate() + hitbox.offset;
                let hitbox_min = hitbox_pos - hitbox.size / 2.0;
                let hitbox_max = hitbox_pos + hitbox.size / 2.0;

                if hitbox_min.x < hurtbox_max.x
                    && hitbox_max.x > hurtbox_min.x
                    && hitbox_min.y < hurtbox_max.y
                    && hitbox_max.y > hurtbox_min.y
                {
                    hurtbox.colliding = true;
                    println!("{} collided with {}", hurtbox.entity, hitbox.entity);
                    hurtbox.iframe = Timer::from_seconds(0.75, TimerMode::Once);

                    if hitbox.projectile {
                        commands.entity(hitbox_entity).despawn();
                    }
                    break;
                }
            }
        }
    }
}

// update hitbox lifetimes
pub fn update_hitbox_lifetimes(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Hitbox)>,
) {
    for (entity, mut hitbox) in query.iter_mut() {
        if let Some(ref mut lifetime) = hitbox.lifetime {
            lifetime.tick(time.delta());
            if lifetime.finished() {
                commands.entity(entity).remove::<Hitbox>();
            }
        }
    }
}
// Color::srgba(1., 0., 0., 0.5)
// System to draw debug visualizations for hitboxes and hurtboxes
pub fn draw_debug_boxes(
    mut gizmos: Gizmos,
    hitbox_query: Query<(&Transform, &Hitbox)>,
    hurtbox_query: Query<(&Transform, &Hurtbox)>,
) {
    for (transform, hitbox) in hitbox_query.iter() {
        let pos = transform.translation.truncate() + hitbox.offset;
        gizmos.rect_2d(pos, 0.0, hitbox.size, Color::srgba(1., 0., 0., 0.5));
    }

    for (transform, hurtbox) in hurtbox_query.iter() {
        let pos = transform.translation.truncate() + hurtbox.offset;
        gizmos.rect_2d(pos, 0.0, hurtbox.size, Color::srgba(0., 1., 0., 0.5));
    }
}

// Helper function to draw an AABB with the given color
fn draw_aabb(gizmos: &mut Gizmos, aabb: Aabb2d, transform: &Transform, color: Color) {
    let position = transform.translation.truncate();
    let scale = transform.scale.truncate();
    let min = aabb.min * scale + position;
    let max = aabb.max * scale + position;

    gizmos.rect_2d(
        (min + max) / 2.0, // center point
        0.0,               // rotation (in radians)
        max - min,         // size
        color,
    );
}

// Helper function to transform an AABB by a given transform
fn transform_aabb(aabb: Aabb2d, transform: &Transform) -> Aabb2d {
    let position = transform.translation.truncate();
    let scale = transform.scale.truncate();

    Aabb2d::new(aabb.min * scale + position, aabb.max * scale + position)
}

// Helper function to check if two AABBs overlap
fn aabbs_overlap(a: Aabb2d, b: Aabb2d) -> bool {
    a.min.x < b.max.x && a.max.x > b.min.x && a.min.y < b.max.y && a.max.y > b.min.y
}

pub fn create_hitbox(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec2,
    lifetime: Option<f32>,
    entity_type: i32,
    projectile: bool,
    enemy: bool,
) {
    let lifetime_timer = lifetime.map(|duration| Timer::from_seconds(duration, TimerMode::Once));
    commands.entity(entity).insert(Hitbox {
        size,
        offset,
        lifetime: lifetime_timer,
        entity: entity_type,
        projectile,
        enemy,
    });
}

pub fn create_hurtbox(
    commands: &mut Commands,
    entity: Entity,
    size: Vec2,
    offset: Vec2,
    entity_type: i32,
    enemy: bool,
) {
    commands.entity(entity).insert(Hurtbox {
        size,
        offset,
        entity: entity_type,
        colliding: false,
        iframe: Timer::from_seconds(1., TimerMode::Once),
        enemy,
    });
}

pub fn get_aabb(size: Vec2, offset: Vec2) -> Aabb2d {
    let half_size = size / 2.0;
    let aabb = Aabb2d::new(offset - half_size, offset + half_size);

    aabb
}
