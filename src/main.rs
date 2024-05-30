use tokio::net::UnixDatagram;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    const SOCKET_PATH: &str = "/tmp/socket_in.sock";
    const BUFFER_SIZE: usize = 200_000;

    let client_socket = UnixDatagram::unbound().unwrap();
    client_socket.connect(SOCKET_PATH)?;

    let mut buffer = vec![0; BUFFER_SIZE];
    loop {
        // Wait for the socket to be writable
        client_socket.writable().await?;

        // Try to send data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match client_socket.try_send(&buffer) {
            Ok(_n) => {
                //break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}