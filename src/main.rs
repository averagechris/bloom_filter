extern crate bloom;
use bloom::*;

pub fn main() {
    let mut bf = BloomFilter::new(10000);
    bf.add(&1200);
    println!("12 is in set: {:?}", bf.query(&12));
    println!("1200 is in set: {:?}", bf.query(&1200));
}
