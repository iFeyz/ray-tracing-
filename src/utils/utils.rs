use rand::Rng;

pub fn random_double() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}