extern crate bloom;
use bloom::*;

pub fn main() {
    let mut bf = BloomFilter::new(5000);
    let num = 12;
    let strn = "hello world".to_string();

    bf.insert(&num);
    bf.insert(&strn);

    assert!(bf.contains(&12));
    assert_eq!(bf.contains(&1200), false);

    assert!(bf.contains(&strn));

    let bunch_of_stuff = vec![100; 2000];

    bf.insert_each(&bunch_of_stuff);
    assert!(bf.contains(&100));
}
