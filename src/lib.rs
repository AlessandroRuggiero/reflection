mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{plugin_fn, FnResult};
#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<String> {
    let mut out = String::new();
    //let candles = fin_data.get_candles("symbol_1")?;
    // for candle in candles {
    //     out += &format!(
    //         "{} {} {} {} {} {}\n",
    //         candle.timestamp,
    //         candle.open,
    //         candle.high,
    //         candle.low,
    //         candle.close,
    //         candle.volume
    //     );
    // }
    out += &format!("Labels: {:?}\n", fin_data.get_labels());
    Ok(out)
}