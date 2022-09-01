use anyhow::Result as AnyResult;
use bytes::Bytes;
use hyper::{
    header::{self, CONTENT_TYPE},
    http::HeaderValue,
    Body, Method, Request, Response, StatusCode,
};
use std::{convert::Infallible, net::IpAddr};

use crate::geo;

#[derive(Debug, Clone)]
pub struct Router {
    geoip: geo::GeoipService,
}

impl Router {
    pub fn new(geoip_service: geo::GeoipService) -> Self {
        Self {
            geoip: geoip_service,
        }
    }

    pub async fn new_router(&mut self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        // println!("{}", req.uri().path());

        let url = req.uri().path();

        if !url.starts_with("/geoip") {
            let response_body = Body::from("");
            let mut response = Response::new(response_body);
            *response.status_mut() = StatusCode::NOT_FOUND;

            return Ok(response);
        }

        if !url.starts_with("/geoip/") {
            let response = Response::new(Body::from("use /geoip/{ip}"));
            return Ok(response);
        }

        let mut parts = url.split("/");
        let ip_string = match parts.nth(2) {
            None => {
                let response = Response::new(Body::from("failed to split"));
                return Ok(response);
            }
            Some(ip_string) => ip_string,
        };

        let ip = match ip_string.parse::<IpAddr>() {
            Err(e) => {
                let response = Response::new(Body::from("err in parsing ip"));
                return Ok(response);
            }
            Ok(ip) => ip,
        };

        let geodata = match self.geoip.get_ip(ip) {
            Err(e) => {
                let response = Response::new(Body::from("err in geoip service"));
                return Ok(response);
            }
            Ok(geodata) => geodata,
        };

        let geodata_parsed = match serde_json::to_string(&geodata) {
            Err(e) => {
                let response = Response::new(Body::from("err in marshaling geodata"));
                return Ok(response);
            }
            Ok(geodata_parsed) => geodata_parsed,
        };

        let mut response = Response::new(Body::from(geodata_parsed));
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(response)
    }
}
