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
    let new_velocity = physics::RigidBody::rigid_body_velocity(&mut ball, dt);
    let new_position = physics::RigidBody::rigid_body_position(&mut ball,dt);

   println!("New Position: {:?}", new_position);
    println!("New Velocity: {:?}", new_velocity);
}
