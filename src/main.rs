#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]

extern crate chrono;
extern crate chrono_tz;

mod timestamp;

use std::env;
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
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Binary usage : tz-range-converter <startTime> <endTime?> <timezone>");
        println!("Rustc usage : cargo run <startTime> <endTime?> <timezone>");
        std::process::exit(1);
    }

    let times :TimestampRange = timestamp::converter::parse_time_range(&args);
    let tz: Tz = args.last().unwrap().as_str().parse().unwrap();

    let t1 = timestamp_to_local(times.start, tz);
    if !times.end.is_empty(){
        println!("{} {}->Local",t1.time(),tz.name());
        return;
    }
    let t2 = timestamp_to_local(times.end, tz);
    println!("{} - {} {}->Local",t1.time(),t2.time(),tz.name());
}

#[cfg(test)]
mod tests {
    use crate::{timestamp::converter::TimestampRange,timestamp_to_local};
    use chrono_tz::Tz;

    #[test]
    fn test_main() {
        let test1: Vec<String> = vec!["11pm".to_string(), "4am".to_string(), "Asia/Tokyo".to_string()];
        let times:TimestampRange = crate::timestamp::converter::parse_time_range(&test1);
        let tz: Tz = test1.last().unwrap().as_str().parse().unwrap();
        let t1 = timestamp_to_local(times.start, tz);
        let t2 = timestamp_to_local(times.end, tz);
        assert_eq!(format!("{} - {} {}->Local",t1.time(),t2.time(),tz.name()), "23:30:00 - 04:30:00 Asia/Tokyo->Local");

        let test2 = vec!["23:00-04:00".to_string(), "Europe/Berlin".to_string()];
        let times:TimestampRange = crate::timestamp::converter::parse_time_range(&test2);
        let tz: Tz = test2.last().unwrap().as_str().parse().unwrap();
        let t1 = timestamp_to_local(times.start, tz).time();
        assert_eq!(format!("{} {}->Local",t1,tz.name()), "06:30:00 Europe/Berlin->Local");

        // Can't handle country code yet
        /* let test3 = vec!["23-4".to_string(), "ET".to_string()];
        let times:TimestampRange = crate::timestamp::converter::parse_time_range(&test3);
        let tz: Tz = test3.last().unwrap().as_str().parse().unwrap();
        let t1 = timestamp_to_local(times.start, tz).time();
        let t2 = timestamp_to_local(times.end, tz).time();
        assert_eq!(format!("{} - {} {}->Local",t1,t2,tz.name()), ""); */

        let test4 = vec!["11:45pm".to_string(),"4:15am".to_string(), "US/Alaska".to_string()];
        let times:TimestampRange = crate::timestamp::converter::parse_time_range(&test4);
        let tz: Tz = test4.last().unwrap().as_str().parse().unwrap();
        let t1 = timestamp_to_local(times.start, tz).time();
        let t2 = timestamp_to_local(times.end, tz).time();
        assert_eq!(format!("{} - {} US/Alaska->Local",t1,t2), "05:15:00 - 21:45:00 US/Alaska->Local");
    }
}
