extern crate geohash;

use geo::Coord;
use geohash::encode;

fn main() {
    let c = Coord {
        x: 112.5584f64,
        y: 37.8324f64,
    };

    let geohash = encode(c, 9usize).unwrap();
    println!("geohash: {}", geohash);
}
