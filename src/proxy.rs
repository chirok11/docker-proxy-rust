use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpSocket, TcpStream};

pub(crate) async fn tcp_listen_proxy(listen_ip: String, listen_port: u16, target_ip: String, target_port: u16) -> Result<bool, std::io::Error> {
    let listener = TcpListener::bind(format!("{}:{}", listen_ip, listen_port)).await?;

    while let Ok((tcp_stream, _socket_addr)) = listener.accept().await {
        let target_ip = target_ip.clone();
        tokio::spawn(async move {
            debug!("new incoming connection");
            process_socket(tcp_stream, target_ip, target_port).await.unwrap()
        });
    }

    Ok(true)
}

async fn process_socket(mut socket: TcpStream, target_ip: String, target_port: u16) -> Result<(u64, u64), std::io::Error> {
    debug!("connecting");
    let mut endpoint = TcpStream::connect(format!("{}:{}", target_ip, target_port)).await?;
    debug!("connected to endpoint {}:{}", target_ip, target_port);
    let (r, w) = tokio::io::copy_bidirectional(&mut socket, &mut endpoint).await?;
    debug!("copied {}:{}", r, w);

    Ok((r, w))
}