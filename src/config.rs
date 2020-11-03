pub struct Config {
    pub ratio: f64,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub quality: bool,
    pub bytes_per_pixel: usize,
    pub time0: f64,
    pub time1: f64,
}

impl Config {
    pub fn speed() -> Config {
        let ratio: f64 = 16.0 / 9.0;
        let width: usize = 800;
        let height: usize = (width as f64 / ratio) as usize;
        let samples_per_pixel: u32 = 100;
        let max_depth: u32 = 50;

        Config {
            ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
            quality: false,
            bytes_per_pixel: 3,
            time0: 0.0,
            time1: 0.0,
        }
    }

    pub fn quality(moving: bool) -> Config {
        let ratio: f64 = 3.0 / 2.0;
        let width: usize = 1200;
        let height: usize = (width as f64 / ratio) as usize;
        let samples_per_pixel: u32 = 500;
        let max_depth: u32 = 50;
        let time0 = 0.0;
        let time1 = if moving { 1.0 } else { 0.0 };

        Config {
            ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
            quality: true,
            bytes_per_pixel: 3,
            time0,
            time1,
        }
    }
}
