use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

use std::thread::sleep;
use std::time::Duration;

extern crate systemstat;
use systemstat::{Platform, System};

use std::process::Command;

fn desktop() -> String {
    let output = Command::new("/usr/bin/bspc")
        .arg("query")
        .arg("-D")
        .arg("-d")
        .arg("focused")
        .arg("--names")
        .output()
        .expect("failed to execute process");

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn battery() -> i32 {
    let sys = System::new();

    match sys.battery_life() {
        Ok(battery) => (battery.remaining_capacity * 100.0) as i32,
        Err(_) => 0,
    }
}

fn main() {
    loop {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        println!(
            " {} %{{c}}{} %{{r}} {:?}% ",
            desktop(),
            datetime.format("%R - %a, %b %d"),
            battery()
        );
        sleep(Duration::from_secs(1));
    }
}
