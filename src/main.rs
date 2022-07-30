#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

extern crate chrono;
extern crate chrono_tz;

mod timestamp;

use std::vec;
use chrono::{TimeZone,prelude::*};
use chrono_tz::Tz;
use timestamp::converter::{Timestamp,TimestampRange};

fn timestamp_to_local(timestamp : Timestamp, tz : Tz) -> DateTime<Local>{
    let local_dt: DateTime<Local> = Local::now();

    let mut hours = timestamp.hours.parse().unwrap();
    let mut minutes : u32 = 0;
    if !timestamp.minutes.is_empty(){
        minutes = timestamp.minutes.parse().unwrap();
    }
    if timestamp.meridiem == "pm"{
        hours += 12;
    }
    let received_dt: DateTime<Tz>= tz.ymd(local_dt.year(), local_dt.month(), local_dt.day()).and_hms(hours, minutes, 0);
    received_dt.with_timezone(&local_dt.timezone())
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let test1: Vec<String> = vec!["11pm".to_string(), "4am".to_string(), "Asia/Tokyo".to_string()];
    /*let test2 = vec!["23:00-04:00", "Europe/Berlin"];
    let test3 = vec!["23-4", "ET"];
    let test4 = vec!["11:45pm","4:15am", "US/Alaska"]; */
    let times :TimestampRange = timestamp::converter::parse_time_range(&test1);
    let tz: Tz = test1.last().unwrap().as_str().parse().unwrap();

    let t1 = timestamp_to_local(times.start, tz);
    let t2 = timestamp_to_local(times.end, tz);
    println!("{} - {} local time",t1.time(),t2.time());
}
