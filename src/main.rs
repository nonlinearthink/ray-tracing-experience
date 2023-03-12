extern crate nalgebra as na;
// extern crate image;

mod ray;

use image::{ImageBuffer, Rgb};
use na::{Point3, Vector3};
use ray::Ray;

fn ray_color(ray: Ray) -> Vector3<f64> {
    let normallized_direction = ray.direction.normalize();
    let t = 0.5 * (normallized_direction.y + 1.);
    return (1. - t) * Vector3::<f64>::new(1., 1., 1.) + t * Vector3::<f64>::new(0.5, 0.7, 1.);
}

fn generate_image() {
    let aspect_ratio = 16. / 9.;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = Point3::<f64>::origin();
    let horizontal = Vector3::<f64>::new(viewport_width, 0., 0.);
    let vertical = Vector3::<f64>::new(0., viewport_height, 0.);
    let left_corner =
        origin - horizontal / 2. - vertical / 2. - Vector3::<f64>::new(0., 0., focal_length);

    let mut imgbuf = ImageBuffer::new(width, height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f64 / (width as f64 - 1.);
        let v = y as f64 / (height as f64 - 1.);
        let ray = Ray::new(origin, left_corner + u * horizontal + v * vertical - origin);
        let color = ray_color(ray);
        *pixel = Rgb([
            (color.x * 255.) as u8,
            (color.y * 255.) as u8,
            (color.z * 255.) as u8,
        ]);
    }
    imgbuf.save("demo.png").unwrap();
}

fn main() {
    generate_image();
}
