// we create a vec3 class
//used for representing geometric vectors and colors

use std::ops;


#[derive(Debug, Default, Copy, Clone, PartialEq)]

// an array of three floating point numbers
pub struct Vec3 {
    e: [f32;3]
}

impl Vec3 {
    //a function that returns vec3 
    pub fn new(e0: f32, e1: f32, e2: f32, ) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    //implementing getters and setters for the vector
    pub fn x(self) -> f32 {
        self.e[0]
    }
    pub fn y(self) -> f32 {
        self.e[1]
    }    
    pub fn z(self) -> f32 {
        self.e[2]
    }

    //and rgb
    pub fn r(self) -> f32 {
        self.e[0]
    }
    pub fn g(self) -> f32 {
        self.e[1]
    }    
    pub fn b(self) -> f32 {
        self.e[2]
    }



    //length calculator 
    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }


    //unit vector 
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    } 

    //dot product
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    //Squared length
    pub fn squared_length(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }


}

//used for summing all the elements and returning a vector 
impl ops::Add for Vec3{
    type Output = Self;

    fn add(self, _right_hand: Vec3) -> Self::Output {

        Vec3 {
            e: [self.e[0] + _right_hand.e[0],
                self.e[1] + _right_hand.e[1],
                self.e[2] + _right_hand.e[2]]
        }

    }

}

//implementing subtract
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _right_hand: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - _right_hand.e[0],
                self.e[1] - _right_hand.e[1],
                self.e[2] - _right_hand.e[2]]
        }
    }
}

//implementing multiply
// vec in float
impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, _right_hand:f32) -> Self::Output{
        Vec3 {
            e:[
                self.e[0] * _right_hand, 
                self.e[1] * _right_hand,
                self.e[2] * _right_hand]
        }

    }
} 
// float in vec
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _right_hand:Vec3) -> Self::Output{
        Vec3 {
            e:[
                _right_hand.e[0] * self, 
                _right_hand.e[1] * self,
                _right_hand.e[2] * self]
        }

    }
} 


//implementing divide
impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, _right_hand: f32) -> Self::Output {

        Vec3 {
            e: [self.e[0] / _right_hand, self.e[1] / _right_hand, self.e[2] / _right_hand]
        }
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(Vec3::new(1.0, 4.0, 3.0 ) + Vec3::new(3.0, 1.0, 2.0),
                    Vec3::new(4.0, 5.0, 5.0))

    }

    #[test]
    fn test_multiply() {
        assert_eq!(Vec3::new(1.0, 4.0, 3.0 ) * 2.0,
                    Vec3::new(2.0, 8.0, 6.0))

    }

    #[test]
    fn test_divide() {
        assert_eq!(Vec3::new(2.0, 4.0, 3.0 ) / 2.0,
                    Vec3::new(1.0, 2.0, 1.5))

    }

}
