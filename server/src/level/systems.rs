use crate::data::gameworld_data::*;
use crate::level::components::*;
use crate::network::components::*;
use bevy::prelude::*;
use bevy::utils::hashbrown::Equivalent;
use rand::Rng;

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
        0.,
    );

    //vec4 to store the ocean level
    let mut tile_map: Vec<OceanTile> = Vec::new();

    while (h as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_H {
        while (w as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_W {
            // weigh it so that its mostly dark blue just for aesthetic reasons
            let rand = rng.gen_range(0..=10);
            if rand < 9 {
                tile_index = 0
            } else {
                tile_index = 1
            }

            //adding tile to the tilemap
            tile_map.push(OceanTile::new(t, tile_index));

            //incrementing
            w += 1;
            t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
        }

        //incrementing
        w = 0;
        t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
        t.x = -OCEAN_W_CENTER + (TILE_SIZE * 2) as f32 / 2.0;
        h += 1;
    }

    return tile_map;
}

/*   SEND_OVERWORLD_DATA FUNCTION   */
/// Sends the information for the ocean overworld level
pub fn send_overworld_data(connections: Res<TcpResource>, ocean_map: Res<OceanMap>) {
    //checking if a new client requested the overworld

    /*
    for stream in connections.streams.lock().unwrap().streams.iter() {
        //checking if client requested overworld data
        let mut buf = [0; 1024];
        stream.peek(&mut buf).expect("Not recieved");
        let request: Packet<String> = serde_json::from_slice(&buf[..1024]).unwrap();

        if request.message.equivalent(&String::from("load"))
            && request.payload.equivalent(&String::from("ocean"))
        {}
    }*/
}
