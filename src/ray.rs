// the ray class that will work on the vector class 

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]

pub struct Ray {
    
    _a: Vec3,
    _b: Vec3,

}

//the ray constructer
impl Ray{
    pub fn ray(a: Vec3, b: Vec3) -> Ray{

        Ray{ _a: a, _b: b }

    }

    //The origin function returns a vector 
    pub fn origin(self) -> Vec3 {
        self._a 
    }
    
    //The direction function
    pub fn direction(self) -> Vec3{
        self._b 
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3{
        self._a + self._b * t 

        //multiply override in vec3
    }


}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_origin() {

    }

    #[test]
    fn test_direction() {

    }

    #[test]
    fn test_point_at_parameter() {

    }
}