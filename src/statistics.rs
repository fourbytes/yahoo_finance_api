use serde::Deserialize;
use time::OffsetDateTime;

use super::YahooError;
use crate::utils::{IntegerValue, DecimalValue, OffsetDateTimeValue};

time::serde::format_description!(iso8601_date, Date, "[year repr:full]-[month repr:numerical]-[day padding:zero]");

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct YStatisticsResponse {
    pub quote_summary_store: Option<YQuoteSummaryStore>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YQuoteSummaryStore {
    pub default_key_statistics: KeyStatistics,
    pub financial_data: Option<FinancialData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyStatistics {
    pub enterprise_value: Option<IntegerValue>,
    pub float_shares: Option<IntegerValue>,
    pub held_percent_insiders: Option<DecimalValue>,
    pub held_percent_institutions: Option<DecimalValue>,
    pub most_recent_quarter: Option<OffsetDateTimeValue>,
    pub net_income_to_common: Option<IntegerValue>,
    pub next_fiscal_year_end: Option<OffsetDateTimeValue>,
    pub price_to_book: Option<DecimalValue>,
    pub shares_outstanding: Option<IntegerValue>,
    pub total_assets: Option<IntegerValue>,
    pub trailing_eps: Option<DecimalValue>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FinancialData {
    pub current_ratio: Option<DecimalValue>,
    pub debt_to_equity: DecimalValue,
    pub ebitda: IntegerValue,
    pub financial_currency: Option<String>,
    pub free_cashflow: IntegerValue,
    pub operating_cashflow: IntegerValue,
    pub quick_ratio: DecimalValue,
    pub return_on_assets: DecimalValue,
    pub total_cash: IntegerValue,
    pub total_debt: IntegerValue,
    pub total_revenue: IntegerValue,
}


impl YStatisticsResponse {
    pub fn from_json(json: serde_json::Value) -> Result<YStatisticsResponse, YahooError> {
        serde_json::from_value(json).map_err(|e| YahooError::DeserializeFailed(e.to_string()))
    }
}
