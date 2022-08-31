use anyhow::Result as AnyResult;
use geohash::{encode, Coordinate};
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

        if let Some(city) = city.city {
            geodata.city.geo_name_id = city.geoname_id.unwrap_or_default();

            if let Some(names_tree) = city.names {
                geodata.city.name_en = names_tree.get("en").unwrap_or(&"s").to_string();
            }
        };

        if let Some(location) = city.location {
            if location.latitude != None && location.longitude != None {
                let latitude = location.latitude.unwrap();
                let longitude = location.longitude.unwrap();

                geodata.location.latitude = latitude;
                geodata.location.longitude = longitude;

                let c = Coordinate {
                    x: latitude,
                    y: longitude,
                };

                geodata.geohash = encode(c, 10_usize).unwrap_or_default();
            }

            geodata.location.time_zone = location.time_zone.unwrap_or("").to_string();
            geodata.location.accuracy_radius = location.accuracy_radius.unwrap_or_default();
        }

        if let Some(traits) = city.traits {
            geodata.traits.is_anonymous_proxy = traits.is_anonymous_proxy.unwrap_or_default();
            geodata.traits.is_satellite_provider = traits.is_satellite_provider.unwrap_or_default();
        }

        geodata
    }
}
