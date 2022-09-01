use anyhow::Result as AnyResult;
use geohash::{encode, Coordinate};
use geoip::model::Subdivision;
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
                geodata.city.name_en = names_tree.get("en").unwrap_or(&"").to_string();
            }
        };

        if let Some(continent) = city.continent {
            geodata.continent.code = continent.code.unwrap_or_default().to_string();
            geodata.continent.geo_name_id = continent.geoname_id.unwrap_or_default();
            if let Some(names_tree) = continent.names {
                geodata.continent.name_en = names_tree.get("en").unwrap_or(&"").to_string();
            }
        }

        if let Some(country) = city.country {
            geodata.country.geo_name_id = country.geoname_id.unwrap_or_default();
            geodata.country.is_in_european_union = country.is_in_european_union.unwrap_or_default();
            geodata.country.iso_code = country.iso_code.unwrap_or_default().to_string();
            if let Some(names_tree) = country.names {
                geodata.continent.name_en = names_tree.get("en").unwrap_or(&"").to_string();
            }
        }

        if let Some(location) = city.location {
            geodata.location.time_zone = location.time_zone.unwrap_or("").to_string();
            geodata.location.accuracy_radius = location.accuracy_radius.unwrap_or_default();

            if location.latitude != None && location.longitude != None {
                let latitude = location.latitude.unwrap();
                let longitude = location.longitude.unwrap();

                geodata.location.latitude = latitude;
                geodata.location.longitude = longitude;

                geodata.geohash = encode(
                    Coordinate {
                        x: latitude,
                        y: longitude,
                    },
                    10_usize,
                )
                .unwrap_or_default();
            }
        }

        if let Some(registered_country) = city.registered_country {
            geodata.registered_country.geo_name_id =
                registered_country.geoname_id.unwrap_or_default();
            geodata.registered_country.iso_code =
                registered_country.iso_code.unwrap_or_default().to_string();

            if let Some(names_tree) = registered_country.names {
                geodata.registered_country.name_en =
                    names_tree.get("en").unwrap_or(&"").to_string();
            }
        }

        let mut subdivisions_models = Vec::<model::Subdivision>::new();

        if let Some(subdivisions) = city.subdivisions {
            for sub in subdivisions {
                let mut subdivision_model = model::Subdivision::default();
                subdivision_model.geo_name_id = sub.geoname_id.unwrap_or_default();
                subdivision_model.iso_code = sub.iso_code.unwrap_or_default().to_string();
                if let Some(names_tree) = sub.names {
                    subdivision_model.name_en = names_tree.get("en").unwrap_or(&"").to_string();
                }

                subdivisions_models.push(subdivision_model);
            }
        }
        geodata.subdivisions = Box::new(subdivisions_models);

        if let Some(traits) = city.traits {
            geodata.traits.is_anonymous_proxy = traits.is_anonymous_proxy.unwrap_or_default();
            geodata.traits.is_satellite_provider = traits.is_satellite_provider.unwrap_or_default();
        }

        geodata
    }
}
