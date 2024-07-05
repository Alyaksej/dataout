use tokio::net::UnixDatagram;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    const SOCKET_PATH: &str = "/home/user/RustroverProjects/server/data-volume/socket_data.sock";
    const BUFFER_SIZE: usize = 212_765;

    let client_socket = UnixDatagram::unbound().unwrap();
    client_socket.connect(SOCKET_PATH)?;

    let mut buffer = vec![0; BUFFER_SIZE];
    let mut next_pkt_num = 0;

    for i in 0..BUFFER_SIZE {
        buffer[i] = i as u8;
    }
    loop {
        // Wait for the socket to be writable
        client_socket.writable().await?;

        // Try to send data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match client_socket.send(&buffer).await {
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
    }

    Ok(())
}