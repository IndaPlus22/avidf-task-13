use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    left_corner: Vec3,
    horizontal: Vec3,
    vertical:Vec3,

}


//the camera with a fixed state in the world
impl Camera {
    pub fn camera() -> Camera{
        Camera {
            left_corner: Vec3::new(-2.0, -1.0, -1.0),
             horizontal: Vec3::new(4.0, 0.0, 0.0),
             vertical: Vec3::new(0.0, 2.0, 0.0),
             origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }


    //keeps shooting rays for us to u and v
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::ray(
            self.origin, 
            self.left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

}