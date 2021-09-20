use color_eyre::Report;
use futures::{stream::FuturesUnordered, StreamExt};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

// Connect using raw TCP connection and writing `GET /` as raw bytes to
// `url`. Then, write the response to a string buffer and log.
async fn fetch_thing(url: &'static str) -> Result<(), Report> {
    let addr: SocketAddr = ([192, 168, 49, 1], 8000).into();
    let mut socket = TcpStream::connect(addr).await?;

    socket.write_all(b"GET / HTTP/1.1\r\n").await?;
    socket.write_all(b"HOST: 1.1.1.1\r\n").await?;
    socket.write_all(b"User-Agent: dolpheyn\r\n").await?;
    socket.write_all(b"Connection: close\r\n").await?;
    socket.write_all(b"\r\n").await?;

    let mut response = String::with_capacity(256);
    socket.read_to_string(&mut response).await?;

    // Status code is on the second line of a HTTP response
    let status = response.lines().next().unwrap_or_default();
    println!("{}", response);
    info!(%status, %url, "Got response!");

    Ok(())
}

// Setup env variables for logging and call trace for async processes
fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let mut futs = vec![fetch_thing(URL_1), fetch_thing(URL_2)]
        .into_iter()
        .collect::<FuturesUnordered<_>>();

    while let Some(item) = futs.next().await {
        item?;
    }

    Ok(())
}
