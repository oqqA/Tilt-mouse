use std::net::UdpSocket;

pub fn start_server(runfn: fn(&std::net::SocketAddr, &[u8; 1500])) {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    std::thread::spawn(move || {
        loop {
            let mut buf = [0u8; 1500];
            match socket.recv_from(&mut buf) {
                Ok((_, src)) => {
                    std::thread::spawn(move || {
                        runfn(&src, &buf);
                    });
                },
                Err(e) => {
                    eprintln!("couldn't recieve a datagram: {}", e);
                }
            }
        }
    });
}