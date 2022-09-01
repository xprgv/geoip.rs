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
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let arguments = Args::parse();
    // println!("{:?}", arguments);

    let geoip_service = geo::GeoipService::new(&arguments.mmdb_path)?;
    let router = router::Router::new(geoip_service);

    let addr: SocketAddr = match arguments.addr.parse() {
        Ok(addr) => addr,
        Err(e) => {
            println!("Error in parsing address: {:?}", e);
            std::process::exit(1);
        }
    };
    // let addr: SocketAddr = arguments.addr.parse().unwrap();
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn: &AddrStream| {
        let router = router.clone();

        async {
            Ok::<_, String>(service_fn(move |req: Request<Body>| {
                let mut router = router.clone();
                async move { Ok::<_, Infallible>(router.new_router(req).await?) }
            }))
        }
    });

    println!("Listen server on {:?}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // println!("done");
    Ok(())
}
