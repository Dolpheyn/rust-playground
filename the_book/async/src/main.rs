use hyper:: {
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, Uri,
};
use std::net::SocketAddr;

async fn serve_req (_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Got request.");
    let url = "http://www.rust-lang.org/en-US/".parse::<Uri>().expect("failed to parse URL");
    let res = Client::new().get(url).await?;
    println!("Got the page, sending it");
    Ok(res)
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr)
        .serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(serve_req))
        }));

    if let Err(_) = serve_future.await {
        eprintln!("Error starting server!");
    }
}


#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    run_server(addr).await;
}
