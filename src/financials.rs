use std::collections::HashMap;

use serde::Deserialize;

use super::YahooError;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct YFinancialsResponse {
    pub quote_time_series_store: YQuoteTimeSeriesStore
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YQuoteTimeSeriesStore {
    pub time_series: TimeSeries
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    pub annual_basic_average_shares: Vec<AnnualBasicAverageShares>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnnualBasicAverageShares {
    // pub as_of_date: chrono::NaiveDate,
    pub currency_code: String,
    pub reported_value: ReportedValue,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportedValue {
    pub raw: i64,
}

impl YFinancialsResponse {
    pub fn from_json(json: serde_json::Value) -> Result<YFinancialsResponse, YahooError> {
        serde_json::from_value(json).map_err(|e| YahooError::DeserializeFailed(e.to_string()))
    }

    pub fn shares_on_issue(&self) -> Option<i64> {
        self.quote_time_series_store.time_series.annual_basic_average_shares
            .last()
            .map(|s| s.reported_value.raw)
    }
}
