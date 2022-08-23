use std::net::IpAddr;

use anyhow::Result as AnyResult;
use maxminddb::{geoip2, MaxMindDBError};

#[derive(Debug)]
pub struct GeoipService {
    reader: maxminddb::Reader<Vec<u8>>,
}

impl GeoipService {
    pub fn new(path: &str) -> Result<GeoipService, MaxMindDBError> {
        let reader = maxminddb::Reader::open_readfile(path)?;

        Ok(GeoipService { reader: reader })
    }

    // pub fn process_ip(ip: IpAddr) -> Result<
}
