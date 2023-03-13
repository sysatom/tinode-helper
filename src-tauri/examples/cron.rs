use cron::Schedule;
use chrono::{Local};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

fn main() {
    //               sec  min   hour   day of month   month   day of week   year
    let expression = "*/3 * * * * *";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("Upcoming fire times:");

    loop {
        let now = Local::now();
        if let Some(next) = schedule.upcoming(Local).next() {
            let delay = next - now;
            thread::sleep(Duration::from_secs(delay.num_seconds() as u64));
            println!("{}", now);
        }
    }
}

/*
Upcoming fire times:
-> 2018-06-01 09:30:00 UTC
-> 2018-06-01 12:30:00 UTC
-> 2018-06-01 15:30:00 UTC
-> 2018-06-15 09:30:00 UTC
-> 2018-06-15 12:30:00 UTC
-> 2018-06-15 15:30:00 UTC
-> 2018-08-01 09:30:00 UTC
-> 2018-08-01 12:30:00 UTC
-> 2018-08-01 15:30:00 UTC
-> 2018-08-15 09:30:00 UTC
*/