use hyper::{header::CONTENT_TYPE, http::HeaderValue, Body, Request, Response, StatusCode};
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

    pub async fn route_request(
        &mut self,
        req: Request<Body>,
    ) -> Result<Response<Body>, Infallible> {
        let mut segments = Vec::new();
        for s in req.uri().path().split('/') {
            match s {
                "" | "." => {}
                ".." => {
                    segments.pop();
                }
                s => segments.push(s),
            }
        }

        match segments[..] {
            ["geoip", ip_string] => self.handle_geoip(ip_string),
            _ => {
                let mut response = Response::new(Body::from(""));
                *response.status_mut() = StatusCode::NOT_FOUND;
                Ok(response)
            }
        }
    }

    fn handle_geoip(&mut self, ip_string: &str) -> Result<Response<Body>, Infallible> {
        let ip = match ip_string.parse::<IpAddr>() {
            Err(_) => {
                let response = Response::new(Body::from("err in parsing ip"));
                return Ok(response);
            }
            Ok(ip) => ip,
        };

        let geodata = match self.geoip.get_ip(ip) {
            Err(_) => {
                let response = Response::new(Body::from("err in geoip service"));
                return Ok(response);
            }
            Ok(geodata) => geodata,
        };

        let geodata_parsed = match serde_json::to_string(&geodata) {
            Err(_) => {
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
