use std::net::UdpSocket;

fn main() {
    println!("Starting Server");

    let socket = UdpSocket::bind("127.0.0.1:4000").unwrap();

    println!("Server listening on {}", socket.local_addr().unwrap());

    let mut buffer = [0; 1024];

    loop {
        let result = socket.recv_from(&mut buffer);
        match result {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let request = String::from_utf8_lossy(&buffer[..size]);

                println!("Received {} from {}", request, source);
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
            }
        }
    }
}
