mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use serde::Serialize;

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    candles: String
}

fn timestamp_to_human_readable(timestamp_ms: i64) -> String {
    let timestamp_s = timestamp_ms / 1000;
    let days_since_epoch = timestamp_s / 86400;
    let seconds_today = timestamp_s % 86400;
    
    // Days since Unix epoch (1970-01-01) to 2000-01-01
    let days_to_2000 = 10957;
    let mut year = 2000;
    let mut days = days_since_epoch - days_to_2000;
    
    // Calculate year
    while days >= 365 {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if is_leap { 366 } else { 365 };
        if days >= days_in_year {
            days -= days_in_year;
            year += 1;
        } else {
            break;
        }
    }
    
    // Calculate month and day
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let days_in_months = if is_leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    
    let mut month = 1;
    for &days_in_month in &days_in_months {
        if days >= days_in_month {
            days -= days_in_month;
            month += 1;
        } else {
            break;
        }
    }
    let day = days + 1;
    
    // Calculate time
    let hours = seconds_today / 3600;
    let minutes = (seconds_today % 3600) / 60;
    let seconds = seconds_today % 60;
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hours, minutes, seconds)
}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    let mut out = String::new();
    for l in fin_data.get_labels() {
        let candles = fin_data.get_candles(l).unwrap();
        for candle in candles {
            let human_readable_date = timestamp_to_human_readable(candle.timestamp);
            out += &format!(
                "{} {} {} {} {} {}\n",
                l,
                human_readable_date,
                candle.open,
                candle.high,
                candle.low,
                candle.close
            );
        }
    }
    Ok (Output { candles: out.to_string() })
}