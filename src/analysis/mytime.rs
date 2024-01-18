use std::fmt;
use std::fmt::Formatter;
use chrono;
use chrono::{Datelike, Timelike, TimeZone};
use chrono_tz::Asia::Ho_Chi_Minh;

// hour_minute range, use the range like the [a, b), i.e. a<= x < b
static DAY_VIEW: (u32, u32) = (730, 1930);
static NIGHT_VIEW_1: (u32, u32) = (1930, 2359);
static NIGHT_VIEW_2: (u32, u32) = (0, 730);
static DAY_START: (u32, u32) = (7, 30);
static NIGHT_START: (u32, u32) = (19, 30);

static DAY_SHIFT_HOURS: [(i32, i32); 13] = [(7, 30), (8, 30), (9, 30), (10, 30), (11, 30), (12, 30),
    (13, 30), (14, 30), (15, 30), (16, 30), (17, 30), (18, 30), (19, 30)];
static NIGHT_SHIFT_HOURS: [(i32, i32); 13] = [(19, 30), (20, 30), (21, 30), (22, 30), (23, 30),
    (0, 30), (1, 30), (2, 30), (3, 30), (4, 30), (5, 30), (6, 30), (7, 30)];


pub fn hours_str(shift: Shift, sum: bool) -> Vec<String> {
    let mut r = Vec::new();
    let hours = match shift {
        Shift::Day => DAY_SHIFT_HOURS,
        Shift::Night => NIGHT_SHIFT_HOURS
    };
    for i in 0..12 {
        r.push(format!("{:2}:{:02} - {:2}:{:02}", hours[i].0, hours[i].1, hours[i + 1].0, hours[i + 1].1));
    }
    if sum {
        r.push("SUM".to_string());
    }
    r
}

#[derive(Debug, Clone, Copy)]
pub enum Shift {
    Day,
    Night,
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Shift::Day => write!(f, "DAY"),
            Shift::Night => write!(f, "NIGHT"),
        }
    }
}

pub struct DateShift(pub i32, pub u32, pub u32, pub Shift);

impl DateShift {
    pub fn date(&self) -> String {
        format!("{}-{:02}-{:02}", self.0, self.1, self.2)
    }
    pub fn shift(&self) -> Shift {
        self.3
    }

    pub fn pre_day(&self) -> Self {
        let (year, month, day) = pre_day_from_int(self.0, self.1, self.2);
        DateShift(year, month, day, self.3)
    }
    pub fn pre_shift(&self) -> Self {
        match self.3 {
            Shift::Day => {
                let (year, month, day) = pre_day_from_int(self.0, self.1, self.2);
                DateShift(year, month, day, Shift::Night)
            }
            Shift::Night => DateShift(self.0, self.1, self.2, Shift::Day)
        }
    }
}

impl fmt::Display for DateShift {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {}", self.0, self.1, self.2, self.3)
    }
}


pub fn now_vn() -> String {
    chrono::Utc::now().with_timezone(&Ho_Chi_Minh).format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn pre_day_from_int(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let day = chrono::Utc.with_ymd_and_hms(year, month, day, 8, 0, 0).unwrap();
    let dur = chrono::Duration::days(1);
    let pre = day - dur;
    (pre.year(), pre.month(), pre.day())
}

pub fn pre_day_str2date(year: &str, month: &str, day: &str) -> String {
    let year = year.parse::<i32>().unwrap();
    let month = month.parse::<u32>().unwrap();
    let day = day.parse::<u32>().unwrap();
    let day = chrono::Utc.with_ymd_and_hms(year, month, day, 8, 0, 0).unwrap();
    let dur = chrono::Duration::days(1);
    let pre = day - dur;
    format!("{}-{:02}-{:02}", pre.year(), pre.month(), pre.day())
}

pub fn pre_shift_str2date<'a>(year: &str, month: &str, day: &str, shift: &str) -> (String, &'a str) {
    match shift {
        "NIGHT" => (format!("{year}-{month}-{day}"), "DAY"),
        "DAY" => {
            let year = year.parse::<i32>().unwrap();
            let month = month.parse::<u32>().unwrap();
            let day = day.parse::<u32>().unwrap();
            let day = chrono::Utc.with_ymd_and_hms(year, month, day, 8, 0, 0).unwrap();
            let dur = chrono::Duration::days(1);
            let pre = day - dur;
            (format!("{}-{:02}-{:02}", pre.year(), pre.month(), pre.day()), "NIGHT")
        }
        _ => unreachable!("pre_day_str2date error")
    }
}

pub fn current_shift() -> DateShift {
    let t = chrono::Utc::now().with_timezone(&Ho_Chi_Minh);
    let (year, month, day, hour, minute) = (t.year(), t.month(), t.day(), t.hour(), t.minute());
    let h_m = hour * 100 + minute;
    if DAY_VIEW.0 <= h_m && h_m < DAY_VIEW.1 {
        DateShift(year, month, day, Shift::Day)
    } else if NIGHT_VIEW_1.0 <= h_m && h_m < NIGHT_VIEW_1.1 {
        DateShift(year, month, day, Shift::Night)
    } else if NIGHT_VIEW_2.0 <= h_m && h_m < NIGHT_VIEW_2.1 {
        let pre_day = pre_day_from_int(year, month, day);
        DateShift(pre_day.0, pre_day.1, pre_day.2, Shift::Night)
    } else {
        unreachable!("current_shift calc error!");
    }
}

pub fn start_end_of_shift(shift: &DateShift) -> (i64, i64) {
    let (h, m) = match shift.3 {
        Shift::Day => DAY_START,
        Shift::Night => NIGHT_START
    };
    let start_time = Ho_Chi_Minh.with_ymd_and_hms(shift.0, shift.1, shift.2, h, m, 0).unwrap();
    let start_time_stamp = start_time.timestamp();
    (start_time_stamp, start_time_stamp + 12 * 3600)
}

pub fn ts_per_hour_shift(shift: &DateShift) -> [(u32, u32); 12] {
    let (h, m) = match shift.3 {
        Shift::Day => DAY_START,
        Shift::Night => NIGHT_START
    };
    let start_time = Ho_Chi_Minh.with_ymd_and_hms(shift.0, shift.1, shift.2, h, m, 0).unwrap();
    let start_time_stamp = start_time.timestamp() as u32;
    let mut r = [(0, 0); 12];
    for i in 0..12 {
        r[i] = (start_time_stamp + i as u32 * 3600, start_time_stamp + i as u32 * 3600 + 3600)
    }
    r
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use super::*;

    #[test]
    fn test_1() {
        // cargo test -- --nocapture
        println!("{}, {}", Shift::Day, Shift::Night);
        assert_eq!(Shift::Day.to_string(), "DAY");
        assert_eq!(Shift::Night.to_string(), "NIGHT");
        println!("current time vn : {}", now_vn());
        println!("{:?}", pre_day_from_int(2024, 1, 1));
        println!("{:?}", pre_day_from_int(2022, 3, 1));
        println!("{:?}", pre_day_from_int(2024, 3, 1));
        assert_eq!(pre_day_from_int(2024, 1, 1), (2023, 12, 31));
        assert_eq!(pre_day_from_int(2022, 3, 1), (2022, 2, 28));
        assert_eq!(pre_day_from_int(2024, 3, 1), (2024, 2, 29));
        println!("current date shift: {}", current_shift());
        println!("start_end_of_shift current: {:?}", start_end_of_shift(&current_shift()));
        println!("horus ts: {:?}", ts_per_hour_shift(&current_shift()));

        let x = start_end_of_shift(&DateShift(2024, 1, 5, Shift::Day));
        println!("start_end_of_shift 1-5-day: {x:?}", );
        println!("horus ts: {:?}", ts_per_hour_shift(&DateShift(2024, 1, 5, Shift::Day)));
        assert_eq!(x, (1704414600, 1704457800));

        let x = start_end_of_shift(&DateShift(2024, 1, 5, Shift::Night));
        println!("start_end_of_shift 1-5-night: {x:?}", );
        println!("horus ts: {:?}", ts_per_hour_shift(&DateShift(2024, 1, 5, Shift::Night)));
        assert_eq!(x, (1704457800, 1704501000));

        let re_date: Regex = Regex::new(r"^(202\d)-(0[1-9]|1[012])-(0[1-9]|[12]\d|3[01])$").unwrap();
        let r = re_date.captures("2024-01-06").unwrap();
        println!("{:?}", r);
        println!("{:?}", hours_str(Shift::Day, true));
        println!("{:?}", hours_str(Shift::Night, false));
    }
}

