use cgmath::{InnerSpace, MetricSpace};
use cgmath::{Quaternion, Rotation3, Vector3};

pub struct Body {
    pub model: String,
    pub mass: f32,

    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub velocity: Vector3<f32>,

    pub radius: f32,
    pub name: String,
}

pub struct Galaxy {
    pub bodies: Vec<Body>,
    gravitational_constant: f32,
    time_scale: f32,
}

impl Galaxy {
    pub fn new() -> Galaxy {
        Galaxy {
            bodies: vec![
                // Body {
                //     model: String::from("saturn"),
                //     mass: 100000.0, //27069794.4184,
                //     position: Vector3::new(0.0, 0.0, 0.0),
                //     rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                //     velocity: Vector3::new(0.0, 0.0, 0.0),
                //     radius: 10.0,
                //     name: String::from("Sun"),
                // },
                // Body {
                //     model: String::from("jupiter"),
                //     mass: 10000.0, //25833.0763597,
                //     position: Vector3::new(-3000.0, 0.0, 0.0),
                //     rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                //     velocity: Vector3::new(0.0, 0.0, 5.0),
                //     radius: 10.0,
                //     name: String::from("Jupiter"),
                // },
                Body {
                    model: String::from("earth"),
                    mass: 1000.0, //81.2800178621,
                    position: Vector3::new(0.0, 0.0, 0.0),
                    rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                    velocity: Vector3::new(0.0, 0.0, 0.0),
                    radius: 10.0,
                    name: String::from("Earth"),
                },
                Body {
                    model: String::from("moon"),
                    mass: 1.0,
                    position: Vector3::new(30.0, 0.0, 0.0),
                    rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                    velocity: Vector3::new(0.0, 0.0, 8.0),
                    radius: 10.0,
                    name: String::from("Moon"),
                },
                Body {
                    model: String::from("mars"),
                    mass: 20.0, //8.7332818450,
                    position: Vector3::new(200.0, 0.0, 0.0),
                    rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                    velocity: Vector3::new(0.0, 0.0, 3.0),
                    radius: 10.0,
                    name: String::from("Mars"),
                },
                Body {
                    model: String::from("venus"),
                    mass: 66.2386573325,
                    position: Vector3::new(-160.0, 0.0, 0.0),
                    rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                    velocity: Vector3::new(0.0, 0.0, 3.0),
                    radius: 10.0,
                    name: String::from("Venus"),
                },
                Body {
                    model: String::from("mercury"),
                    mass: 4.47080315055,
                    position: Vector3::new(130.0, 0.0, 0.0),
                    rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                    velocity: Vector3::new(0.0, 0.0, 5.0),
                    radius: 10.0,
                    name: String::from("Mercury"),
                },
                // Body {
                //     model: String::from("uranus"),
                //     mass: 1000.0, //1181.46247032,
                //     position: Vector3::new(4000.0, 0.0, 0.0),
                //     rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                //     velocity: Vector3::new(0.0, 0.0, 3.0),
                //     radius: 10.0,
                //     name: String::from("Uranus"),
                // },
                // Body {
                //     model: String::from("neptune"),
                //     mass: 1000.0, //1393.6384859,
                //     position: Vector3::new(-4300.0, 0.0, 0.0),
                //     rotation: Quaternion::from_angle_x(cgmath::Deg(0.0)),
                //     velocity: Vector3::new(0.0, 0.0, 3.0),
                //     radius: 10.0,
                //     name: String::from("Neptune"),
                // },
            ],
            gravitational_constant: 1.6,
            time_scale: 1.0,
        }
    }

    pub fn tick(&mut self) {
        let bodies_count = self.bodies.len();
        for i in 0..bodies_count {
            let mut force = Vector3::new(0.0, 0.0, 0.0);
            for j in 0..bodies_count {
                if i == j {
                    continue;
                }
                force += -(self.gravitational_constant * self.bodies[i].mass * self.bodies[j].mass
                    / self.bodies[i].position.distance2(self.bodies[j].position))
                    * (self.bodies[i].position - self.bodies[j].position).normalize();
            }
            let body = &mut self.bodies[i];
            body.velocity += force / body.mass;
            body.position += body.velocity;
        }
    }
}
