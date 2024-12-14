mod physics;
use physics::RigidBody;
#[allow(unused_imports)]
use my_project_lib::MyVec;
fn main() {
    let mut ball = RigidBody::initialize();

    println!("Initial Position: {:?}", ball.position);
    println!("Initial Velocity: {:?}", ball.velocity);

   // const GRAVITY: MyVec = MyVec::new(0.0, -9.8);
    let dt = 5.0;
    let new_velocity = physics::RigidBody::update_rigid_body_velocity(&mut ball, dt);
    let new_position = physics::RigidBody::update_rigid_body_position(&mut ball,dt);
    let new_acceleration = physics::RigidBody::update_rigid_body_acceleration(&mut ball);
    let new_angular_velocity = physics::RigidBody::update_angular_velocity(&mut ball, dt);
    let new_linear_momentum = physics::RigidBody::rigid_body_linear_momentum(&ball);
    let new_angular_momentum = physics::RigidBody::rigid_body_angular_momemtum(&mut ball, dt);

   println!("New Position: {:?}", new_position);
   println!("New Velocity: {:?}", new_velocity);
   println!("New Acceleration: {:?}", new_acceleration);
   //println!("New Momentum: {:?}", new_momentum);
   println!("New Angular Veclocity: {:?}", new_angular_velocity);
   println!("New Linear Momentum: {:?}", new_linear_momentum);
   println!("New Angular Momentum: {:?}", new_angular_momentum);

}
