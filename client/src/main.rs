use std::net::UdpSocket;

fn main() {
    
    println!("Starting Client");

    let socket = UdpSocket::bind("127.0.0.1:8000")
        .unwrap();

    println!("Client listening on {}", socket.local_addr().unwrap());

    let server = "127.0.0.1:4000";

    let mut buffer = [0; 1024];

    let mut request = "Hello there, I am a client!";

    socket.send_to(request.as_bytes(), server)
            .unwrap();
        
    let (size, source) = socket.recv_from(&mut buffer).unwrap();

    let server_response = String::from_utf8_lossy(&buffer[..size]);

    println!("Server {} responded: {}", source, server_response);

    let request = "Okay, goodbye Mr. Server!";

    socket.send_to(request.as_bytes(), server)
        .unwrap();

    println!("Client process closing...");
}
