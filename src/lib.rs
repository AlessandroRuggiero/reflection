mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{encoding, plugin_fn, FnResult, Json, ToBytes};
use serde::Serialize;
use chrono::{self, DateTime};

#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    candles: String
}

#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    let mut out = String::new();
    for l in fin_data.get_labels() {
        let candles = fin_data.get_candles(l).unwrap();
        for candle in candles {
            let human_readable_date = DateTime::from_timestamp(candle.timestamp / 1000, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
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