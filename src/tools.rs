use rand::Rng;

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(min, max)
}

pub fn random_double() -> f64 {
    random_double_range(0.0, 1.0)
}

pub fn random_usize_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(min, max)
}
