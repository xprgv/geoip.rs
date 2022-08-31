extern crate geohash;

use std::error::Error;

use geohash::{encode, Coordinate};

fn main() {
    let c = Coordinate {
        x: 112.5584f64,
        y: 37.8324f64,
    };

    let geohash = encode(c, 9usize).unwrap();
    println!("geohash: {}", geohash);
}
