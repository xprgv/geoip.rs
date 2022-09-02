use clap::Parser;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use std::{convert::Infallible, net::SocketAddr};

mod geo;
mod model;
mod router;

#[derive(Debug, Parser)]
#[clap(author = "openmind3d", version = "v0.1.0")]
struct Args {
    #[clap(long)]
    addr: String,

    #[clap(long)]
    mmdb_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let arguments = Args::parse();

    let geoip_service = geo::GeoipService::new(&arguments.mmdb_path)?;
    let router = router::Router::new(geoip_service);

    let addr: SocketAddr = match arguments.addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Error in parsing address: {:?}", e);
            std::process::exit(1);
        }
    };

    let make_svc = make_service_fn(|_conn: &AddrStream| {
        let router = router.clone();

        async {
            Ok::<_, String>(service_fn(move |req: Request<Body>| {
                let mut router = router.clone();
                async move { Ok::<_, Infallible>(router.route_request(req).await?) }
            }))
        }
    });

    println!("Starting geoip service on: {}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
