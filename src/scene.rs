use crate::aarect::XyRect;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::moving_sphere::MovingSphere;
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::tools::{random_double, random_double_range};
use crate::vec3::{Color, Point3, Vec3};

pub struct Config {
    pub ratio: f64,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub time0: f64,
    pub time1: f64,
}

#[derive(Debug, PartialEq)]
pub enum SceneKind {
    RandomUniform,
    RandomChecker,
    TwoCheckerSpheres,
    TwoPerlinSpheres,
    ImageSphere,
    SimpleLight,
}

pub struct Scene {
    pub cfg: Config,
    pub world: Vec<Box<dyn Hittable + Sync>>,
    pub camera: Camera,
    pub background: Color,
}

impl Config {
    fn new(speed: bool, moving: bool) -> Config {
        if speed {
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
                time0: 0.0,
                time1: 0.0,
            }
        } else {
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
                time0,
                time1,
            }
        }
    }
}

impl Scene {
    pub fn new(speed: bool, moving: bool, kind: SceneKind, filename: &str) -> Scene {
        // Image config
        let cfg = Config::new(speed, moving);

        // Camera
        let lookfrom = match kind {
            SceneKind::SimpleLight => Point3::new(26.0, 3.0, 6.0),
            _ => Point3::new(13.0, 2.0, 3.0),
        };

        let lookat = match kind {
            SceneKind::SimpleLight => Point3::new(0.0, 2.0, 0.0),
            _ => Point3::zero(),
        };

        let vup = Vec3::new(0.0, 1.0, 0.0);
        let vfov = 20.0;
        let dist_to_focus = 10.0;
        let aperture = match kind {
            SceneKind::RandomUniform | SceneKind::RandomChecker => 0.1,
            _ => 0.0,
        };
        // FIXME vfov def: 40.0 (20.0 for TwoCheckerSphere and Random*)

        let camera = Camera::new(
            lookfrom,
            lookat,
            vup,
            vfov,
            cfg.ratio,
            aperture,
            dist_to_focus,
            cfg.time0,
            cfg.time1,
        );

        let background = match kind {
            SceneKind::SimpleLight => Color::zero(),
            _ => Color::new(0.7, 0.8, 1.0),
        };

        let mut scene = Scene {
            cfg,
            world: Vec::new(),
            camera,
            background,
        };

        match kind {
            SceneKind::RandomUniform | SceneKind::RandomChecker => {
                scene.create_random(kind);
            }
            SceneKind::TwoCheckerSpheres => scene.create_two_spheres(),
            SceneKind::TwoPerlinSpheres => scene.create_two_perlin_spheres(),
            SceneKind::ImageSphere => scene.create_image_sphere(filename),
            SceneKind::SimpleLight => scene.create_simple_light(),
        }

        scene
    }

    fn create_simple_light(&mut self) {
        // ground
        let noise = NoiseTexture::from(4.0);
        let material_noise = Lambertian {
            albedo: Box::new(noise),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(material_noise),
        }));

        // sphere
        let noise = NoiseTexture::from(4.0);
        let material_noise = Lambertian {
            albedo: Box::new(noise),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(material_noise),
        }));

        // Rectangle light
        let difflight = DiffuseLight::from(Color::new(4.0, 4.0, 4.0));

        self.world.push(Box::new(XyRect {
            x0: 3.0,
            x1: 5.0,
            y0: 1.0,
            y1: 3.0,
            k: -2.0,
            material: Box::new(difflight),
        }));
    }

    fn create_image_sphere(&mut self, filename: &str) {
        // sphere
        let texture = ImageTexture::new(filename);
        let material = Lambertian {
            albedo: Box::new(texture),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::zero(),
            radius: 2.0,
            material: Box::new(material),
        }));
    }

    fn create_two_perlin_spheres(&mut self) {
        // ground
        let noise = NoiseTexture::from(4.0);
        let material_noise = Lambertian {
            albedo: Box::new(noise),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(material_noise),
        }));

        // sphere
        let noise = NoiseTexture::from(4.0);
        let material_noise = Lambertian {
            albedo: Box::new(noise),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(material_noise),
        }));
    }

    fn create_two_spheres(&mut self) {
        let checker = CheckerTexture::from((Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
        let material_ground = Lambertian {
            albedo: Box::new(checker),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, -10.0, 0.0),
            radius: 10.0,
            material: Box::new(material_ground),
        }));

        let checker = CheckerTexture::from((Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
        let material_ground = Lambertian {
            albedo: Box::new(checker),
        };
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, 10.0, 0.0),
            radius: 10.0,
            material: Box::new(material_ground),
        }));
    }

    fn create_random(&mut self, kind: SceneKind) {
        // ground
        match kind {
            SceneKind::RandomUniform => {
                let material_ground = Lambertian::from(Color::new(0.5, 0.5, 0.5));
                self.world.push(Box::new(Sphere {
                    center: Point3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                    material: Box::new(material_ground),
                }));
            }
            SceneKind::RandomChecker => {
                let checker =
                    CheckerTexture::from((Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
                let material_ground = Lambertian {
                    albedo: Box::new(checker),
                };
                self.world.push(Box::new(Sphere {
                    center: Point3::new(0.0, -1000.0, 0.0),
                    radius: 1000.0,
                    material: Box::new(material_ground),
                }));
            }
            _ => panic!("Invalid kind expect one of the Random ones: {:?}", kind),
        }

        // random part
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_double();
                let center = Point3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    // shared_ptr<material> sphere_material;

                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Lambertian::from(albedo);
                        if self.cfg.time0 == self.cfg.time1 {
                            // simple sphere
                            self.world.push(Box::new(Sphere {
                                center,
                                radius: 0.2,
                                material: Box::new(sphere_material),
                            }));
                        } else {
                            // moving sphere
                            let center2 =
                                center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                            self.world.push(Box::new(MovingSphere {
                                center0: center,
                                center1: center2,
                                time0: 0.0,
                                time1: 1.0,
                                radius: 0.2,
                                material: Box::new(sphere_material),
                            }));
                        }
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_double_range(0.0, 0.5);
                        let sphere_material = Metal::new(albedo, fuzz);
                        self.world.push(Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Box::new(sphere_material),
                        }));
                    } else {
                        // glass
                        let sphere_material = Dielectric::new(1.5);
                        self.world.push(Box::new(Sphere {
                            center,
                            radius: 0.2,
                            material: Box::new(sphere_material),
                        }));
                    }
                }
            }
        }

        // fixed part
        let material1 = Dielectric::new(1.5);
        self.world.push(Box::new(Sphere {
            center: Point3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(material1),
        }));

        let material2 = Lambertian::from(Color::new(0.4, 0.2, 0.1));
        self.world.push(Box::new(Sphere {
            center: Point3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(material2),
        }));

        let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
        self.world.push(Box::new(Sphere {
            center: Point3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(material3),
        }));
    }
}
