mod mousecursorapi;
mod udpserver;
mod excelwriter; // for analysis sensor data

fn main() {
    mousecursorapi::set_cursor_pos(10,10); // for test
    udpserver::start_server(use_data_from_server);
    loop {
        // code desctop app ...
    }
}

fn use_data_from_server(src: &std::net::SocketAddr, buf: &[u8; 1500]) {
    println!("{} say: {}",src, std::str::from_utf8(buf).expect("Could not write buffer as string"));
}