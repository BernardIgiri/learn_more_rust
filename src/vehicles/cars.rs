use glam::Vec2;

const STEER_TIME: f32 = 15.0;

#[derive(Debug)]
pub enum State {
    Idle,
    Parked,
    Driving,
    Turning(Vec2),
}

#[derive(Debug)]
pub struct Car {
    name: String,
    power: f32,
    braking: f32,
    mass: f32,
    state: State,
    velocity: Vec2,
    heading: Vec2,
    max_speed: f32,
    interpolation: f32,
}

impl Car {
    pub fn new_ferrari() -> Car {
        Car {
            name: "Ferrari".to_string(),
            power: 5.0,
            braking: 5.0,
            mass: 1.0,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 100.0,
            interpolation: 0.0,
        }
    }
    pub fn new_mustang() -> Car {
        Car {
            name: "Mustang".to_string(),
            power: 3.0,
            braking: 4.0,
            mass: 1.35,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 80.0,
            interpolation: 0.0,
        }
    }
    pub fn new_sedan() -> Car {
        Car {
            name: "Sedan".to_string(),
            power: 2.0,
            braking: 3.0,
            mass: 1.5,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 75.0,
            interpolation: 0.0,
        }
    }
    pub fn new_pickup_truck() -> Car {
        Car {
            name: "Pickup Truck".to_string(),
            power: 2.5,
            braking: 3.0,
            mass: 3.0,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 70.0,
            interpolation: 0.0,
        }
    }
    pub fn new_go_kart() -> Car {
        Car {
            name: "Go Kart".to_string(),
            power: 0.5,
            braking: 1.0,
            mass: 0.2,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 40.0,
            interpolation: 0.0,
        }
    }
    pub fn new_mercedes() -> Car {
        Car {
            name: "Mercedes".to_string(),
            power: 3.0,
            braking: 3.0,
            mass: 2.0,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 90.0,
            interpolation: 0.0,
        }
    }
}

impl Car {
    pub fn animate(&mut self, t: f32) {
        match self.state {
            State::Idle => self.stop(t),
            State::Parked => self.stop(t),
            State::Driving => self.accelerate(t),
            State::Turning(heading) => self.turn_to(heading, t),
        }
    }
    pub fn set_heading(&mut self, v: &Vec2) {
        self.heading = v.normalize()
    }
    pub fn set_state(&mut self, s: State) {
        self.state = s;
        self.interpolation = 0.0;
    }
    pub fn velocity(&self) -> &Vec2 {
        &self.velocity
    }
    pub fn heading(&self) -> &Vec2 {
        &self.heading
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Car {
    fn stop(&mut self, t: f32) {
        self.accelerate_by(-self.braking / self.mass, t);
    }
    fn accelerate(&mut self, t: f32) {
        self.accelerate_by(self.power / self.mass, t);
    }
    fn accelerate_by(&mut self, a: f32, t: f32) {
        let delta = self.heading * a * t;
        self.velocity = if a < 0.0 && delta.length() > self.velocity.length() {
            Vec2::zero()
        } else {
            self.velocity + delta
        };
        if self.velocity.length() > self.max_speed {
            self.velocity = self.heading * self.max_speed;
        }
    }
    fn turn_to(&mut self, heading: Vec2, t: f32) {
        let delta = t / STEER_TIME + self.interpolation;
        self.interpolation = if delta > 1.0 { 1.0 } else { delta };
        self.velocity = self
            .velocity
            .lerp(heading * self.velocity.length(), self.interpolation);
        self.set_heading(&self.velocity.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    fn test_car() -> Car {
        Car {
            name: "test".to_string(),
            power: 1.0,
            braking: 1.0,
            mass: 5.0,
            state: State::Parked,
            velocity: Vec2::new(0.0, 0.0),
            heading: Vec2::new(1.0, 0.0),
            max_speed: 50.0,
            interpolation: 0.0,
        }
    }

    #[test]
    fn car_stop_lowers_velocity() {
        let mut car = Car {
            velocity: Vec2::new(2.0, 0.0),
            state: State::Driving,
            ..test_car()
        };
        car.stop(0.2);
        assert_eq!(car.velocity.y(), 0.0);
        assert!(approx_eq!(f32, car.velocity.x(), 1.96, ulps = 2));
    }

    #[test]
    fn car_stop_does_not_pass_zero() {
        let mut car = Car {
            velocity: Vec2::new(2.0, 0.0),
            state: State::Driving,
            ..test_car()
        };
        car.stop(2000.0);
        assert_eq!(car.velocity.y(), 0.0);
        assert_eq!(car.velocity.x(), 0.0);
    }

    #[test]
    fn car_accelerate_increases_velocity() {
        let mut car = test_car();
        car.accelerate(0.2);
        assert_eq!(car.velocity.y(), 0.0);
        assert!(approx_eq!(f32, car.velocity.x(), 0.04, ulps = 2));
    }

    #[test]
    fn car_accelerate_does_not_pass_max_speed() {
        let mut car = test_car();
        car.accelerate(2000.0);
        assert_eq!(car.velocity.y(), 0.0);
        assert_eq!(car.velocity.x(), 50.0);
    }
}
