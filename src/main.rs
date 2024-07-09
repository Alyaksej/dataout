use tokio::net::UnixDatagram;
use std::{fs, io};
use tokio::io::Interest;

#[tokio::main]
async fn main() -> io::Result<()> {
    const SOCKET_DATA_PATH: &str = "/tmp/socket_data.sock";
    const SOCKET_RESULT_PATH: &str = "/tmp/socket_result.sock";
    const BUFFER_SIZE: usize = 212_765;

    if fs::metadata(SOCKET_RESULT_PATH).is_ok() {
        if let Err(e) = fs::remove_file(SOCKET_RESULT_PATH) {
            eprintln!("Error removing socket file: {}", e);
            return Err(e);
        }
    };
    // Create sockets
    let socket_result = match UnixDatagram::bind(SOCKET_RESULT_PATH) {
        Ok(socket_result) => socket_result,
        Err(e) => {
            eprintln!("Error binding socket data: {}", e);
            return Err(e);
        }
    };

    let socket_data = UnixDatagram::unbound()?;

    let mut buffer = vec![0; BUFFER_SIZE];
    let mut next_pkt_num = 0;
    let mut buf = vec![0; 99_999];

    for i in 0..BUFFER_SIZE {
        buffer[i] = i as u8;
    }
    loop {
        // Wait for the socket to be writable
        socket_data.writable().await?;

        // Try to send data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match socket_data.try_send_to(&buffer, SOCKET_DATA_PATH) {
            Ok(n) => {
                buffer[0] = next_pkt_num;
                if next_pkt_num == 255 {
                    next_pkt_num = 0;
                } else {
                    next_pkt_num += 1;
                }
                if n != BUFFER_SIZE {
                    return Err(io::Error::new(io::ErrorKind::Other, "buffer is not equal to n"));
                };
                //break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //println!("!!WouldBlock");
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }

        match socket_result.try_recv_from(&mut buf) {
            Ok((len_result, addr)) => {
                println!("socket_result len_result: {}, addr {:?}", len_result, addr);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //println!("socket_result WouldBlock");
            }
            Err(e) => {
                eprintln!("Error receiving data: {:?}", e);
            }
        }
    }
    Ok(())
}