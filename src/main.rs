use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

use std::time::Duration;
use std::thread::sleep;

extern crate systemstat;
use systemstat::{System, Platform};

fn battery() -> i32 {
    let sys = System::new();

    match sys.battery_life() {
        Ok(battery) =>
            (battery.remaining_capacity * 100.0) as i32,
        Err(_) =>
            0
    }
}

fn main() {
    loop {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        println!(" %{{c}}{} %{{r}} {:?}% ", datetime.format("%R - %a, %b %d"), battery());
        sleep(Duration::from_secs(1));
    }
}
