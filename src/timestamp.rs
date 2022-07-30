pub mod converter {
    use regex::Regex;
    use regex::Captures;

    #[derive(Debug,Clone)]
    pub struct Timestamp {
        pub minutes:String, 
        pub hours:String,
        pub meridiem:String,
    }

    impl Timestamp {
        fn new()->Timestamp {
            Timestamp { minutes: "".to_string(), hours: "".to_string(), meridiem: "".to_string() }
        }    
    }

    #[derive(Debug)]
    pub struct TimestampRange {
        pub start: Timestamp ,
        pub end: Timestamp ,
    }

    impl TimestampRange {
        fn new()->TimestampRange {
            TimestampRange {
                start: Timestamp::new() ,
                end: Timestamp::new() ,
            }
        } 
    }

    fn capture_to_timestamp(caps : Captures) -> Timestamp{
        let mut ts = Timestamp::new();
        ts.hours = caps.get(1).map_or("".to_string(), |m| m.as_str().to_string());
        if ts.hours.is_empty(){
            ts.hours = caps.get(4).map_or("".to_string(), |m| m.as_str().to_string());
        }
        ts.meridiem = caps.get(2).map_or("".to_string(), |m| m.as_str().to_string());
        ts.minutes = caps.get(3).map_or("".to_string(), |m| m.as_str().to_string());
        ts
    }

    pub fn parse_time_range(times : &[String]) -> TimestampRange{

        let re = Regex::new(r"(\d{1,2})[:h]?(am|pm)?(\d{2})?(am|pm)?").unwrap();

        let mut parsed = Vec::new();  

        for time in times.iter().take(2) {
            let caps : Captures = re.captures(time.as_str()).unwrap();
            let timestamp :Timestamp = capture_to_timestamp(caps);
            parsed.push(timestamp);
        }

        let mut res = TimestampRange::new();
        res.start = parsed[0].clone();
        res.end = parsed[1].clone();
        res
    }
}
