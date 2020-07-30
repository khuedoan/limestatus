use std::time::SystemTime;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

use std::time::Duration;
use std::thread;

fn main() {
    loop {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        println!("{}", datetime.format("%R - %a, %b %d"));
        thread::sleep(Duration::from_secs(1));
    }
}
