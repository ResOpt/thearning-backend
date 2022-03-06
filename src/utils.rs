use rand::Rng;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn generate_random_id() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>().abs()
}
