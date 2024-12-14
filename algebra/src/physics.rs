use core::f64;

use my_project_lib::{my_vec, MyVec};
#[allow(dead_code)]

pub struct RigidBody {
   pub position: MyVec,
   pub velocity: MyVec,
   pub   rotation: f64,
   pub angular_velocity: f64,
   pub mass: f64,
   pub moment_of_inertia: f64,
   pub forces: MyVec,
   pub torque: f64,
   pub vertices: Vec<MyVec>,
   
}
//const GRAVITY: MyVec = MyVec::new(0.0, -9.8);
impl RigidBody {
    
    pub fn initialize () -> RigidBody {
    let _ball = RigidBody {
     position: MyVec::new(0.1, 0.5),
        velocity: MyVec::new(0.6, 0.7),
        rotation: 0.0,
        angular_velocity: 0.0,
        mass: 1.0,
        moment_of_inertia : 0.5, 
        forces: MyVec::new(0.5,0.7),
        torque: 1.0,
        vertices:vec![my_vec(
       1.0 , 1.0)],
        
    };
    _ball
    
}

//pub fn rigid_body_velocity (rigid_body:&mut RigidBody,dt:f64) -> MyVec { // wrong formula 
   // let x_axis = rigid_body.velocity.x * rigid_body.forces.x / rigid_body.mass * dt;
  //  let y_axis = rigid_body.velocity.y * rigid_body.forces.y / rigid_body.mass * dt;
  //  MyVec{
   //     x:x_axis,
   //     y:y_axis,
   // }
//}

pub fn update_rigid_body_position(rigid_body: &mut RigidBody,dt: f64) -> MyVec {
    
    
    MyVec{
        x:rigid_body.position.x + rigid_body.velocity.x * dt,
        y:rigid_body.position.y + rigid_body.velocity.y* dt,
        
    }
}
pub fn update_rigid_body_velocity(rigid_body: &mut RigidBody, dt: f64) -> MyVec {
    // Calculate the acceleration in each axis
    let ax = rigid_body.forces.x / rigid_body.mass;
    let ay = rigid_body.forces.y / rigid_body.mass;

    // Update the velocity based on acceleration and time step
    let new_velocity_x = rigid_body.velocity.x + ax * dt;
    let new_velocity_y = rigid_body.velocity.y + ay * dt;

    // Return the updated velocity
    MyVec {
        x: new_velocity_x,
        y: new_velocity_y,
    }
}

pub fn update_rigid_body_acceleration(&self)-> MyVec {
    MyVec{
        x:self.forces.x / self.mass,
        y:self.forces.y / self.mass,
    }
}

pub fn update_angular_velocity(&mut self, dt: f64) -> f64 {
    
    let angular_acc =  self.torque / self.moment_of_inertia;
    let new_angular_velocity = self.angular_velocity + angular_acc*dt;
    new_angular_velocity
   
}


pub fn rigid_body_linear_momentum(&self) -> MyVec {
    MyVec {
        x: self.mass * self.velocity.x,
        y: self.mass * self.velocity.y,
    }
}

pub fn rigid_body_angular_momemtum(&mut self,dt: f64) -> f64 {
    let angular_mometum =  self.torque * self.update_angular_velocity(dt);
    angular_mometum
   
   
   }
   pub fn project(&self, axis:MyVec) -> MyVec {
    let mut min = f64::NEG_INFINITY;
    let mut max = f64::INFINITY;
    for vertex in &self.vertices {
        let projection = MyVec::dot_product(*vertex, axis);

        if projection < min {
            min = projection;
        }
        if projection > max {
            max = projection;
        }
    }
    MyVec{
        x:min,
        y:max,
    }
    
   }

   fn overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    !(max1 < min2 || max2 < min1)
}


fn separating_axis_theorem(body1: Vec<MyVec>, body2: Vec<MyVec>) -> bool {
    let mut axes = MyVec::new(1.0,1.0);

    // Generate axes from body1
    for i in 0..body1.len() {
        let next = (i + 1) % body1.len();
        let edge = MyVec::subtract(body1[next], body1[i]);
        axes.push(edge.perpendicular_vector());
    }

    // Generate axes from body2
    for i in 0..body2.len() {
        let next = (i + 1) % body2.len();
        let edge = MyVec::subtract(body1[next], body1[i]);
        axes.push(edge.perp());
    }


   

}

}
