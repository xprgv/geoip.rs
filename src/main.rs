use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use std::{convert::Infallible, env, net::SocketAddr};

mod geo;
mod model;
mod router;

const MMDB_PATH: &str = "./assets/GeoLite2-City.mmdb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    let geoip_service = geo::GeoipService::new(MMDB_PATH)?;
    let router = router::Router::new(geoip_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn: &AddrStream| {
        let router = router.clone();

        async {
            Ok::<_, String>(service_fn(move |req: Request<Body>| {
                let mut router = router.clone();
                async move { Ok::<_, Infallible>(router.new_router(req).await?) }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // println!("done");
    Ok(())
}
