pub struct Config {
    pub ratio: f64,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub quality: bool,
}

impl Config {
    pub fn speed() -> Config {
        let ratio: f64 = 16.0 / 9.0;
        let width: u32 = 800;
        let height: u32 = (width as f64 / ratio) as u32;
        let samples_per_pixel: u32 = 100;
        let max_depth: u32 = 50;

        Config {
            ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
            quality: false,
        }
    }

    pub fn quality() -> Config {
        let ratio: f64 = 3.0 / 2.0;
        let width: u32 = 1200;
        let height: u32 = (width as f64 / ratio) as u32;
        let samples_per_pixel: u32 = 500;
        let max_depth: u32 = 50;

        Config {
            ratio,
            width,
            height,
            samples_per_pixel,
            max_depth,
            quality: true,
        }
    }
}
