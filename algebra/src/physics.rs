use my_project_lib::MyVec;
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
}
//const GRAVITY: MyVec = MyVec::new(0.0, -9.8);
impl RigidBody {
    pub fn initialize () -> RigidBody {
    let _ball = RigidBody {
     position: MyVec::new(0.0, 0.0),
        velocity: MyVec::new(0.0, 0.0),
        rotation: 0.0,
        angular_velocity: 0.0,
        mass: 1.0,
        moment_of_inertia : 0.5, 
        forces: MyVec::new(0.0,0.0),
        torque: 0.0,
    };
    _ball
    
}

pub fn rigid_body_velocity (rigid_body:&mut RigidBody,dt:f64) -> MyVec {
    let x_axis = rigid_body.velocity.x * rigid_body.forces.x / rigid_body.mass * dt;
    let y_axis = rigid_body.velocity.y * rigid_body.forces.y / rigid_body.mass * dt;
    MyVec{
        x:x_axis,
        y:y_axis,
    }
}


}

