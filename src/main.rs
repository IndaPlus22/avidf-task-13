

mod vec3;
mod ray;
mod hittable;
mod hittabel_list;
mod sphere;
mod camera;


use vec3::Vec3;
use ray::Ray;
use hittable::{HitRecord, Hittable};
use hittabel_list::HittableList;
use sphere::Sphere;
use camera::Camera; 

//random 
use rand::prelude::*;


//It will draw from left top to right and come down
fn color(r: &Ray, world: &HittableList) -> Vec3 {

    let mut rec = HitRecord::default();

    //is that recursion I see there!
    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p() + rec.normal() + random_in_unit_sphere();
        return 0.5 * color(&Ray::ray(rec.p(), target - rec.p()), &world) 
    }
    else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5*(unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }

}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();

    //keeps looping to the point where p squared length is less than one 
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
        
        if p.squared_length() < 1.0 {
            return p
        }
    }

    
}



fn main() {

    //Defining the genral looks of it
    let x = 300;
    let y = 150;
    let samples = 100; //number of samples 
    let max_value = 255;


    let mut list: Vec<Box<dyn Hittable>>  = Vec::new();
    //first sphere
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5))); 
    //second sphere
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    //implement the camera
    let _camera = Camera::camera();
    let mut rng = rand::thread_rng();


    println!("P3 \n{} {}\n{}", x, y, max_value);

    //Constructs a ppm that loops through the coordiantes
    for j in (0..y).rev() {
        for i in 0..x {

            let mut _color = Vec3::default();

            //Me love rust 
            //me love underscore in loops where nothing being used
            for _ in 0..samples {
                
                let u = (i as f32 + rng.gen::<f32>()) / x as f32;
                let v = (j as f32 + rng.gen::<f32>()) / y as f32;

                let r = &_camera.get_ray(u,v);
                let p = r.point_at_parameter(2.0);
                _color = _color + color(&r, &world);
            }

            _color = _color / samples as f32;

            //using gamma coorrection for color intensity
            _color = Vec3::new(_color.r().sqrt(), _color.g().sqrt(), _color.b().sqrt());


            let ir = (255.99 * _color.r()) as i32;
            let ig = (255.99 * _color.g()) as i32;
            let ib = (255.99 * _color.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }



}