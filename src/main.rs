#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    clippy::unnecessary_wraps
)]
// use chrono::{DateTime, FixedOffset, Local, Utc};
// use std::{env,vec,string};


pub mod converter {
    use regex::Regex;
    use regex::Captures;

    #[derive(Debug)]
    pub struct Timestamp {
        minutes:String, 
        hours:String,
        meridiem:String,
    }

    impl Timestamp {
        fn new()->Timestamp {
            Timestamp { minutes: "".to_string(), hours: "".to_string(), meridiem: "".to_string() }
        }    
    }

    #[derive(Debug)]
    pub struct TimestampRange {
        pub list: Vec<Timestamp >,
    }

    impl TimestampRange {
        fn new()->TimestampRange {
            TimestampRange {
                list: vec![],
            }
        } 
    }

    pub fn parse_time_range(times : Vec<String>) -> TimestampRange{

        let re = Regex::new(r"(\d{1,2})[:h]?(am|pm)?(\d{2})?(am|pm)?").unwrap();

        let mut res = TimestampRange::new();  

        for time in times{
            let cap : Captures = re.captures(time.as_str()).unwrap();
            let timestamp_tuple = (cap.get(1), cap.get(2).or_else( || cap.get(4)), cap.get(3));
            let timestamp : Option<Timestamp>= match timestamp_tuple{
                (Some(hours), Some(meridiem), Some(minutes)) => Some(Timestamp {
                    hours: hours.as_str().to_string(),
                    meridiem: meridiem.as_str().to_string(),
                    minutes: minutes.as_str().to_string(),
                }),
                _ => None,
            };
            res.list.push(timestamp.unwrap());
        }
        res
    }
}

use std::vec;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let test1: Vec<String> = vec!["11pm".to_string(), "4am".to_string(), "Asia/Tokyo".to_string()];
    /*let test2 = vec!["23:00-04:00", "Europe/Berlin"];
    let test3 = vec!["23-4", "ET"];
    let test4 = vec!["11:45pm","4:15am", "US/Alaska"]; */
    let times = converter::parse_time_range(test1);
    println!("{:?}",times.list);

    //let tz : String = times.pop().unwrap_or(""); 

}
