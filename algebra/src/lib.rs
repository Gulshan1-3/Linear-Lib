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

}
