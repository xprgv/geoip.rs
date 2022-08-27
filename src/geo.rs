use anyhow::Result as AnyResult;
use maxminddb::{geoip2, MaxMindDBError};
use std::{net::IpAddr, sync::Arc};

use crate::model;

#[derive(Debug, Clone)]
pub struct GeoipService {
    reader: Arc<maxminddb::Reader<Vec<u8>>>,
}

impl GeoipService {
    pub fn new(path: &str) -> Result<GeoipService, MaxMindDBError> {
        let reader = maxminddb::Reader::open_readfile(path)?;

        Ok(GeoipService {
            reader: Arc::new(reader),
        })
    }

    pub fn get_ip(&mut self, ip: IpAddr) -> AnyResult<model::Geodata> {
        let city: geoip2::City = self.reader.lookup(ip)?;

        let mut geodata = GeoipService::fill_geodata(city);

        geodata.ip = ip.to_string();

        Ok(geodata)
    }

    fn fill_geodata(city: geoip2::City) -> model::Geodata {
        let mut geodata = model::Geodata::default();

        geodata
    }
}
