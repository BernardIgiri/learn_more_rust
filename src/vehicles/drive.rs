use super::cars;
use glam::Vec2;
use std::{thread, time};

fn format_vector(v: &Vec2) -> String {
    format!("({x:0>7.03}, {y:0>7.03})", x = v.x, y = v.y)
}

fn animate_drive<S: Into<String>>(car: &mut cars::Car, label: S, start: u32, end: u32) {
    let delay_time = time::Duration::from_millis(50);
    let label = label.into();
    for n in start..end {
        print!("\x1B[2J\x1B[1;1H");
        let v = format_vector(car.velocity());
        let h = format_vector(car.heading());
        println!(
            "{} performing {} test.\nFrame {}\nVelocity {}\nHeading  {}\n",
            car.name(),
            label,
            n,
            v,
            h,
        );
        thread::sleep(delay_time);
        car.animate(0.23);
    }
}

pub fn test_drive(car: &mut cars::Car) {
    car.set_state(cars::State::Driving);
    animate_drive(car, "acceleration", 1, 100);
    car.set_state(cars::State::Idle);
    animate_drive(car, "braking", 100, 200);
    car.set_state(cars::State::Driving);
    animate_drive(car, "handling", 200, 220);
    car.set_state(cars::State::Turning(Vec2::new(0.0, 1.0)));
    animate_drive(car, "steering", 220, 300);
    car.set_state(cars::State::Parked);
    animate_drive(car, "final braking", 300, 330);
}
