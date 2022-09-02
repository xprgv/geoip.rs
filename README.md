# Geoip

Api service for getting geo information from ip address

## Run
```sh
cargo r -- --addr=127.0.0.1:3000 --mmdb-path=./assets/GeoLite2-City.mmdb
```

## Usage

```sh
curl http://localhost:3000/geoip/176.170.170.190
```

```json
{
    "ip": "176.170.170.190",
    "geohash": "t11wzzzpxg",
    "city": {
        "geo_name_id": 2975204,
        "name_en": "Seloncourt"
    },
    "continent": {
        "code": "EU",
        "geo_name_id": 6255148,
        "name_en": "France"
    },
    "country": {
        "geo_name_id": 3017382,
        "is_in_european_union": true,
        "iso_code": "FR",
        "name_en": ""
    },
    "location": {
        "accuracy_radius": 10,
        "latitude": 47.4599,
        "longitude": 6.8554,
        "time_zone": "Europe/Paris"
    },
    "registered_country": {
        "geo_name_id": 3017382,
        "iso_code": "FR",
        "name_en": "France"
    },
    "subdivisions": [
        {
            "geo_name_id": 11071619,
            "iso_code": "BFC",
            "name_en": "Bourgogne-Franche-Comte"
        },
        {
            "geo_name_id": 3020989,
            "iso_code": "25",
            "name_en": "Doubs"
        }
    ],
    "traits": {
        "is_anonymous_proxy": false,
        "is_satellite_provider": false
    }
}
```