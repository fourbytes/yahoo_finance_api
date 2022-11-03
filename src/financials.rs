use serde::Deserialize;

use super::YahooError;

time::serde::format_description!(iso8601_date, Date, "[year repr:full]-[month repr:numerical]-[day padding:zero]");
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct YFinancialsResponse {
    pub quote_time_series_store: YQuoteTimeSeriesStore
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YQuoteTimeSeriesStore {
    pub time_series: Option<TimeSeries>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    pub annual_basic_average_shares: Vec<Option<AnnualBasicAverageShares>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnnualBasicAverageShares {
    #[serde(with = "iso8601_date")]
    pub as_of_date: time::Date,
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
        self.quote_time_series_store.time_series.as_ref().and_then(|ts| ts.annual_basic_average_shares
            .last()
            .and_then(|s| s.as_ref().map(|s| s.reported_value.raw)))
    }
}
