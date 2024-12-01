use bevy::prelude::*;
use rand::Rng;
use crate::data::gameworld_data::*;
use crate::level::components::*;

/*   BUILD_OCEAN FUNCTION   */
/// Builds the ocean level and stores it in a Vec
pub fn build_ocean() -> Vec<OceanTile> {
    //initiating rng
    let mut rng = rand::thread_rng();
    let mut tile_index;
    
    //creating width and height
    let mut w = 0;
    let mut h = 0;

    //creating vec3 to store tile translation
    let mut t = Vec3::new(
    -OCEAN_W_CENTER + TILE_SIZE as f32 / 2.,
    -OCEAN_H_CENTER + TILE_SIZE as f32 / 2.,
    -1.0,
    );

    //vec4 to store the ocean level
    let mut tile_map: Vec<OceanTile> = Vec::new();

    while (h as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_H {
        while (w as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_W {

            // weigh it so that its mostly dark blue just for aesthetic reasons
            let rand = rng.gen_range(0..=10);
            if rand < 9 { tile_index = 0 }
            else { tile_index = 1 }

            //adding tile to the tilemap
            tile_map.push(OceanTile::new(t, tile_index));
    
            //incrementing
            w += 1;
            t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
        }

        //incrementing
        w = 0;
        t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
        t.x = -OCEAN_W_CENTER + (TILE_SIZE * 2) as f32/2.0;
        h+=1;
    }

    return tile_map;
}