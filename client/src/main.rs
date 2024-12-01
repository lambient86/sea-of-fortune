use std::net::UdpSocket;
use bevy::prelude::*;

fn main() {
    
    println!("Starting Client");

    //connect to server
    let socket = UdpSocket::bind("127.0.0.1:8000")
        .unwrap();

    println!("Client listening on {}", socket.local_addr().unwrap());

    let server = "127.0.0.1:4000"; 

    App::new();
        //.add_systems(Startup, );
}

