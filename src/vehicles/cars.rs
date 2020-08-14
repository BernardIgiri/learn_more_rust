use glam;
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
        return Car {
            name: "Ferrari".to_string(),
            power: 5.0,
            braking: 5.0,
            mass: 1.0,
            state: State::Idle,
            velocity: Vec2::zero(),
            heading: Vec2::unit_x(),
            max_speed: 100.0,
            interpolation: 0.0,
        };
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
