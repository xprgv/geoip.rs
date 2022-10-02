use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Geodata {
    pub ip: String,
    pub geohash: String,
    pub city: City,
    pub continent: Continent,
    pub country: Country,
    pub location: Location,
    pub registered_country: RegisteredCountry,
    pub subdivisions: Vec<Subdivision>,
    pub traits: Traits,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct City {
    pub geo_name_id: u32,
    pub name_en: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Continent {
    pub code: String,
    pub geo_name_id: u32,
    pub name_en: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Country {
    pub geo_name_id: u32,
    pub is_in_european_union: bool,
    pub iso_code: String,
    pub name_en: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Location {
    pub accuracy_radius: u16,
    pub latitude: f64,
    pub longitude: f64,
    pub time_zone: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RegisteredCountry {
    pub geo_name_id: u32,
    pub iso_code: String,
    pub name_en: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Subdivision {
    pub geo_name_id: u32,
    pub iso_code: String,
    pub name_en: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Traits {
    pub is_anonymous_proxy: bool,
    pub is_satellite_provider: bool,
}
