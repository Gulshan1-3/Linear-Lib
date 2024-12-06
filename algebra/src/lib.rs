pub const  fn my_vec(x: f64, y: f64) -> MyVec {
    MyVec::new(x,y)
}
#[derive(Clone, Copy,Debug, PartialEq)]
pub struct MyVec {
   pub x:f64,
    pub y:f64
}

impl MyVec {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn add(vec_a:MyVec,vec_b:MyVec) -> Vec<f64> {
      let answer = vec![vec_a.x + vec_b.x,vec_a.y+ vec_b.y];
      answer     
    }
    
    pub fn subtract(vec_a:MyVec,vec_b:MyVec) -> Vec<f64> {
        let answer = vec![vec_a.x - vec_b.x,vec_a.y - vec_b.y];
        answer     
      }

      pub fn scarlar_multiply(s:f64,vec_a:MyVec) -> Vec<f64> {
        let answer  = vec![s * vec_a.x,s * vec_a.y];
        answer
      }

      pub fn scarlar_division(s:f64,vec_a:MyVec) -> Vec<f64> {
        let answer  = vec![s / vec_a.x,s / vec_a.y];
        answer
      }
   

    pub fn equality(vec_a:MyVec,vec_b:MyVec) -> bool {
      if vec_a.x == vec_b.x && vec_a.y == vec_b.y {
        vec_a == vec_b
      }
      else{
        vec_a != vec_b
      }
    }
  
    pub fn normality (vec1:MyVec) -> Vec<f64> {
      let s = vec1.x * vec1.x + vec1.y * vec1.y;
     let z =  s.sqrt();
      let normal_x = vec1.x / z; 
      let normal_y = vec1.y / z; 
      vec![normal_x, normal_y]
    }
    pub fn angle(vec1:MyVec,vec2:MyVec) -> f64 {
      let mag1 = MyVec::magnitude(vec1);
      let mag2 = MyVec::magnitude(vec2);
      let step = MyVec::dot_product(vec1, vec2);
      let angle = f64::acos(step/mag1*mag2);
      angle 
    }

    pub fn distance_btw_vec(vec1:MyVec,vec2:MyVec) -> f64 {
      let diff = MyVec::subtract(vec2, vec1);
      let dist = MyVec::magnitude(diff);
      dist
    }
    pub fn magnitude(vec_a:MyVec) -> f64 {
      let z =  vec_a.x * vec_a.x + vec_a.y * vec_a.y;
      z.sqrt()
     }
     pub fn perpendicular_vector(vec1:MyVec) -> Vec<f64> {
      let z = vec![-vec1.y,vec1.x];
      z
     }
    pub fn dot_product(vec_a:MyVec,vec_b:MyVec) -> f64 {
        vec_a.x * vec_b.x + vec_a.y * vec_b.y
    } 

    pub fn perp_dot_product(vec_a:MyVec,vec_b:MyVec) -> f64 {
        vec_a.x * vec_b.y - vec_a.y * vec_b.x
    }
}
#[cfg(test)]
mod test {
	use super::*;
	

  #[test]

  fn test_new() {
    let vec = MyVec::new(2., 3.);
    assert_eq!(vec.x,2.);
    println!("Created Vector:{:?}",vec);
    assert_eq!(vec.y,3.0);
  }

  #[test]
fn test_dot_product () {
    let vec_a = MyVec::new(1.0, 2.0);
    let vec_b = MyVec::new(3.0, 4.0);
    let dot = MyVec::dot_product(vec_a,vec_b);
    assert_eq!(dot,11.0);
}

#[test]
fn test_perp_dot_product () {
    let vec_a = MyVec::new(1.0, 2.0);
    let vec_b = MyVec::new(3.0, 4.0);
    let dot = MyVec::perp_dot_product(vec_a,vec_b);
    println!("Vector:{:?}",dot);
    assert_eq!(dot,-2.0);
}

#[test]
fn test_add_fn() {
    let vec_a = MyVec::new(1.0, 2.0);
    let vec_b = MyVec::new(3.0, 4.0);
    let answer = MyVec::add(vec_a, vec_b);
    println!("Vector: {:?}", answer);
    assert_eq!(answer, vec![4.0, 6.0]);
}

#[test]
fn test_subtract_fn() {
    let vec_a = MyVec::new(1.0, 2.0);
    let vec_b = MyVec::new(3.0, 4.0);
    let answer = MyVec::subtract(vec_a, vec_b);
    println!("Vector: {:?}", answer);
    assert_eq!(answer, vec![-2.0, -2.0]);
}
#[test]
fn test_scalar_multiply() {
    let s = 2.; 
    let vec_a = MyVec::new(3.0, 4.0);
    let answer = MyVec::scarlar_multiply(s, vec_a);
    println!("Vector: {:?}", answer);
    assert_eq!(answer, vec![6.,8.]);
}

#[test]
fn test_scalar_division() {
    let s = 4.; 
    let vec_a = MyVec::new(2.0, 4.0);
    let answer = MyVec::scarlar_division(s, vec_a);
    println!("Vector: {:?}", answer);
    assert_eq!(answer, vec![2.,1.]);
}
#[test]
 fn test_magnitude () {
  let vec_a = MyVec::new(3.0,4.0);
  let mag = MyVec::magnitude(vec_a);
  assert_eq!(mag,5.);
 }
 #[test]
 fn test_equality_same_vectors() {
     let vec_a = MyVec::new(1., 2.);
     let vec_b = MyVec::new(1., 2.);
     assert!(MyVec::equality(vec_a, vec_b), "Vectors should be equal.");
 }

 #[test]
 fn test_equality_different_vectors() {
     let vec_a = MyVec::new(1., 4.);
     let vec_b = MyVec::new(3., 2.);
     assert!(!MyVec::equality(vec_a, vec_b), "Vectors should not be equal.");
 }
 #[test]
 fn test_euclidean_distance() {
     // Test case 1: 2D vectors
     let v1 = vec![3.0, 4.0];
     let v2 = vec![0.0, 0.0];
     let result = MyVec::distance_btw_vec(v1, v2);
     assert_eq!(result, 5.0);

}
}