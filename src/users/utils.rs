use rand::{Rng, thread_rng};

pub fn is_email(s: &String) -> bool {
    s.as_str().contains("@")
}


pub fn generate_random_id() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>()
}