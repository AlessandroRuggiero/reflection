mod exchange_outpost;
use crate::exchange_outpost::FinData;
use extism_pdk::{plugin_fn, FnResult};
#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<String> {
    let mut out = String::new();
    let candles = fin_data.get_candles("symbol_1")?;
    out += &format!("len: {}", candles.len());
    Ok(out)
}