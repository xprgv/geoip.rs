use std::net::IpAddr;

use maxminddb::geoip2;

const database_path: &str = "./assets/GeoLite2-City.mmdb";

fn main() {
    let reader = maxminddb::Reader::open_readfile(database_path).expect("Failed to open database");

    let ip: IpAddr = "172.200.116.42".parse().unwrap();
    let city: geoip2::City = reader.lookup(ip).expect("Failed to get ip");

    println!("{:?}", city);
}
