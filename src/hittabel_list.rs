//list to create a vec for all that is hit


use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: & mut HitRecord) -> bool {
        //for temporary record
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_hit = t_max;

        for object in &self.list {
            if object.hit(r, t_min, closest_hit, & mut temp_rec) {
                hit_anything = true;
                closest_hit = temp_rec.t();
                
                //setters for t p normal
                rec.set_t(temp_rec.t());
                rec.set_p(temp_rec.p());
                rec.set_normal(temp_rec.normal());
                
            }
        }
        hit_anything
    }
}