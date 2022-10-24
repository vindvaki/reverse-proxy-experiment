use http::Request;
use hyper::{client::conn, Body};
use tokio::net::{TcpListener, TcpStream};

/// The client starts a TCP server, waits for a connection to open, and then
/// sends HTTP _requests_ back over the connection.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        match listener.accept().await {
            Ok((tcp_stream, _addr)) => {
                handle_connection(tcp_stream).await?; // TODO: Log error
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

async fn handle_connection(tcp_stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let (mut request_sender, connection) = conn::handshake(tcp_stream).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Error in connection: {}", e);
        }
    });

    let request = Request::builder()
        .header("Host", "localhost")
        .method("GET")
        .body(Body::from(""))?;

    let response = request_sender.send_request(request).await?;
    let body_bytes = hyper::body::to_bytes(response.into_body()).await?;
    println!("{:?}", body_bytes);

    // NOTE: To reuse connection, we need to wait for the sender to become ready:
    // request_sender.ready().await?

    Ok(())
}
