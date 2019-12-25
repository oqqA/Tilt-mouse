use std::net::UdpSocket;

pub fn start_server(runfn: fn(&std::net::SocketAddr, &[u8; 1500])) {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Could not bind socket");
    std::thread::spawn(move || {
        loop {
            let mut buf = [0u8; 1500];
            //let sock = socket.try_clone().expect("Failed to clone socket"); // echo server for test
            match socket.recv_from(&mut buf) {
                Ok((_, src)) => {
                    runfn(&src, &buf);
                    //thread::spawn(move || { ... next line ... } ); // echo server for test
                    //sock.send_to(&buf, &src).expect("Failed to send a response"); // echo server for test
                },
                Err(e) => {
                    eprintln!("couldn't recieve a datagram: {}", e);
                }
            }
        }
    });
}