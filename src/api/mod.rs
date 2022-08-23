use crate::geo;
use anyhow::anyhow;
use hyper::{Body, HeaderMap, Method, Request, Response, StatusCode};
use std::convert::Infallible;
use thiserror::Error;

type RequestBody = Request<Body>;
type ResponseBody = anyhow::Result<Response<Body>>;
type ServerResponse = Result<Response<Body>, Infallible>;

#[derive(Error, Debug)]
enum ServerErrors {
    // #[error("")]
    // TokenNotFound,
    #[error("path not found")]
    PathNotFound,
}

#[derive(Debug)]
pub struct Api {
    pub geoip: geo::GeoipService,
}

impl Api {
    pub fn new(geoip_service: geo::GeoipService) -> Self {
        Api {
            geoip: geoip_service,
        }
    }

    // pub async fn new_api(&mut self, req: Request<Body>) -> ServerResponse {}

    // pub(super) async fn geo_router(&mut self, req: RequestBody) -> ResponseBody {
    //     match (req.method(), req.uri().path()) {
    //         (&Method::POST, "/geoip") => self.handle_ip(req).await,
    //         _ => Err!(ServerErrors::PathNotFound),
    //     }
    // }
}
