use rand::Rng;
#[allow(dead_code)]
pub fn gen_random_string() -> String {
    let mut rng = rand::thread_rng();
    let random_str = rng.gen::<u32>().to_string();
    return random_str;
}
