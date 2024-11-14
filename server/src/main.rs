use std::net::UdpSocket;

use bevy::render::render_resource::Buffer;

fn main()  {

    println!("Starting Server");
    
    let socket = UdpSocket::bind("127.0.0.1:4000")
        .unwrap();

    println!("Server listening on {}", socket.local_addr().unwrap());

    let mut buffer = [0; 1024];

    loop {
        let (size, source) = socket.recv_from(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..size]);

        println!("Received {} from {}", request, source);

        let response = "Hello from Server...!";

        socket.send_to(response.as_bytes(), source)
            .unwrap();
    }
}
