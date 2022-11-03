use std::collections::HashMap;

use reqwest::Url;
use time::{Date, OffsetDateTime, UtcOffset};
use async_compat::CompatExt;

use crate::financials::ReportedValue;

use super::*;

impl YahooConnector {
    /// Retrieve the quotes of the last day for the given ticker
    pub async fn get_latest_quotes(
        &self,
        ticker: &str,
        interval: &str,
    ) -> Result<YResponse, YahooError> {
        self.get_quote_range(ticker, interval, "1mo").await
    }

    /// Retrieve the quote history for the given ticker form date start to end (inclusive), if available
    pub async fn get_quote_history(
        &self,
        ticker: &str,
        start: OffsetDateTime,
        end: OffsetDateTime,
    ) -> Result<YResponse, YahooError> {
        self.get_quote_history_interval(ticker, start, end, "1d")
            .await
    }

    /// Retrieve quotes for the given ticker for an arbitrary range
    pub async fn get_quote_range(
        &self,
        ticker: &str,
        interval: &str,
        range: &str,
    ) -> Result<YResponse, YahooError> {
        let url: String = format!(
            YCHART_RANGE_QUERY!(),
            url = self.url,
            symbol = ticker,
            interval = interval,
            range = range
        );
        YResponse::from_json(send_request(&url).await?)
    }
    /// Retrieve the quote history for the given ticker form date start to end (inclusive), if available; specifying the interval of the ticker.
    pub async fn get_quote_history_interval(
        &self,
        ticker: &str,
        start: OffsetDateTime,
        end: OffsetDateTime,
        interval: &str,
    ) -> Result<YResponse, YahooError> {
        let url = format!(
            YCHART_PERIOD_QUERY!(),
            url = self.url,
            symbol = ticker,
            start = start.to_offset(UtcOffset::UTC).unix_timestamp(),
            end = end.to_offset(UtcOffset::UTC).unix_timestamp(),
            interval = interval
        );
        YResponse::from_json(send_request(&url).await?)
    }

    pub async fn get_financials(
        &self,
        ticker: &str
    ) -> Result<YFinancialsResponse, YahooError> {
        let url = format!(
            YFINANCIALS_QUERY!(),
            url = YSCRAPE_URL,
            symbol = ticker
        );
        YFinancialsResponse::from_json(send_scrape_request(&url).await?)
    }

    pub async fn get_statistics(
        &self,
        ticker: &str
    ) -> Result<YStatisticsResponse, YahooError> {
        let url = format!(
            YSTATISTICS_QUERY!(),
            url = YSCRAPE_URL,
            symbol = ticker
        );
        YStatisticsResponse::from_json(send_scrape_request(&url).await?)
    }

    /// Retrieve the list of quotes found searching a given name
    pub async fn search_ticker_opt(&self, name: &str) -> Result<YSearchResultOpt, YahooError> {
        let url = format!(YTICKER_QUERY!(), url = self.search_url, name = name);
        YSearchResultOpt::from_json(send_request(&url).await?)
    }

    /// Retrieve the list of quotes found searching a given name
    pub async fn search_ticker(&self, name: &str) -> Result<YSearchResult, YahooError> {
        let result = self.search_ticker_opt(name).await?;
        Ok(YSearchResult::from_opt(&result))
    }

    /// https://query1.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/NFL.AX?lang=en-AU&region=AU&symbol=NFL.AX&padTimeSeries=true&type=quarterlyTotalAssets,trailingTotalAssets,quarterlyStockholdersEquity,trailingStockholdersEquity,quarterlyGainsLossesNotAffectingRetainedEarnings,trailingGainsLossesNotAffectingRetainedEarnings,quarterlyRetainedEarnings,trailingRetainedEarnings,quarterlyCapitalStock,trailingCapitalStock,quarterlyTotalLiabilitiesNetMinorityInterest,trailingTotalLiabilitiesNetMinorityInterest,quarterlyTotalNonCurrentLiabilitiesNetMinorityInterest,trailingTotalNonCurrentLiabilitiesNetMinorityInterest,quarterlyOtherNonCurrentLiabilities,trailingOtherNonCurrentLiabilities,quarterlyNonCurrentDeferredRevenue,trailingNonCurrentDeferredRevenue,quarterlyNonCurrentDeferredTaxesLiabilities,trailingNonCurrentDeferredTaxesLiabilities,quarterlyLongTermDebt,trailingLongTermDebt,quarterlyCurrentLiabilities,trailingCurrentLiabilities,quarterlyOtherCurrentLiabilities,trailingOtherCurrentLiabilities,quarterlyCurrentDeferredRevenue,trailingCurrentDeferredRevenue,quarterlyCurrentAccruedExpenses,trailingCurrentAccruedExpenses,quarterlyIncomeTaxPayable,trailingIncomeTaxPayable,quarterlyAccountsPayable,trailingAccountsPayable,quarterlyCurrentDebt,trailingCurrentDebt,quarterlyTotalNonCurrentAssets,trailingTotalNonCurrentAssets,quarterlyOtherNonCurrentAssets,trailingOtherNonCurrentAssets,quarterlyOtherIntangibleAssets,trailingOtherIntangibleAssets,quarterlyGoodwill,trailingGoodwill,quarterlyInvestmentsAndAdvances,trailingInvestmentsAndAdvances,quarterlyNetPPE,trailingNetPPE,quarterlyAccumulatedDepreciation,trailingAccumulatedDepreciation,quarterlyGrossPPE,trailingGrossPPE,quarterlyCurrentAssets,trailingCurrentAssets,quarterlyOtherCurrentAssets,trailingOtherCurrentAssets,quarterlyInventory,trailingInventory,quarterlyAccountsReceivable,trailingAccountsReceivable,quarterlyCashCashEquivalentsAndShortTermInvestments,trailingCashCashEquivalentsAndShortTermInvestments,quarterlyOtherShortTermInvestments,trailingOtherShortTermInvestments,quarterlyCashAndCashEquivalents,trailingCashAndCashEquivalents&merge=false&period1=493590046&period2=1667449537&corsDomain=au.finance.yahoo.com
    pub async fn get_financials_timeseries(&self, symbol: &str, period: FinancialsPeriod) -> Result<HashMap<String, Vec<(OffsetDateTime, f64)>>, YahooError> {
        let ts = OffsetDateTime::now_utc().unix_timestamp();
        let mut url: Url = format!("https://query1.finance.yahoo.com/ws/fundamentals-timeseries/v1/finance/timeseries/{symbol}").parse().unwrap();
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("lang", "en-AU");
            query.append_pair("region", "AU");
            query.append_pair("symbol", symbol);
            query.append_pair("padTimeSeries", "true");
            query.append_pair("type", match period {
                FinancialsPeriod::Quarterly => "quarterlyTotalAssets,quarterlyStockholdersEquity,quarterlyGainsLossesNotAffectingRetainedEarnings,quarterlyRetainedEarnings,quarterlyCapitalStock,quarterlyTotalLiabilitiesNetMinorityInterest,quarterlyTotalNonCurrentLiabilitiesNetMinorityInterest,quarterlyOtherNonCurrentLiabilities,quarterlyNonCurrentDeferredRevenue,quarterlyNonCurrentDeferredTaxesLiabilities,quarterlyLongTermDebt,quarterlyCurrentLiabilities,quarterlyOtherCurrentLiabilities,quarterlyCurrentDeferredRevenue,quarterlyCurrentAccruedExpenses,quarterlyIncomeTaxPayable,quarterlyAccountsPayable,quarterlyCurrentDebt,quarterlyTotalNonCurrentAssets,quarterlyOtherNonCurrentAssets,quarterlyOtherIntangibleAssets,quarterlyGoodwill,quarterlyInvestmentsAndAdvances,quarterlyNetPPE,quarterlyAccumulatedDepreciation,quarterlyGrossPPE,quarterlyCurrentAssets,quarterlyOtherCurrentAssets,quarterlyInventory,quarterlyAccountsReceivable,quarterlyCashCashEquivalentsAndShortTermInvestments,quarterlyOtherShortTermInvestments,quarterlyCashAndCashEquivalents",
                FinancialsPeriod::Annual => "annualTotalAssets,annualStockholdersEquity,annualGainsLossesNotAffectingRetainedEarnings,annualRetainedEarnings,annualCapitalStock,annualTotalLiabilitiesNetMinorityInterest,annualTotalNonCurrentLiabilitiesNetMinorityInterest,annualOtherNonCurrentLiabilities,annualNonCurrentDeferredRevenue,annualNonCurrentDeferredTaxesLiabilities,annualLongTermDebt,annualCurrentLiabilities,annualOtherCurrentLiabilities,annualCurrentDeferredRevenue,annualCurrentAccruedExpenses,annualIncomeTaxPayable,annualAccountsPayable,annualCurrentDebt,annualTotalNonCurrentAssets,annualOtherNonCurrentAssets,annualOtherIntangibleAssets,annualGoodwill,annualInvestmentsAndAdvances,annualNetPPE,annualAccumulatedDepreciation,annualGrossPPE,annualCurrentAssets,annualOtherCurrentAssets,annualInventory,annualAccountsReceivable,annualCashCashEquivalentsAndShortTermInvestments,annualOtherShortTermInvestments,annualCashAndCashEquivalents",
                FinancialsPeriod::Trailing => "trailingTotalAssets,trailingStockholdersEquity,trailingGainsLossesNotAffectingRetainedEarnings,trailingRetainedEarnings,trailingCapitalStock,trailingTotalLiabilitiesNetMinorityInterest,trailingTotalNonCurrentLiabilitiesNetMinorityInterest,trailingOtherNonCurrentLiabilities,trailingNonCurrentDeferredRevenue,trailingNonCurrentDeferredTaxesLiabilities,trailingLongTermDebt,trailingCurrentLiabilities,trailingOtherCurrentLiabilities,trailingCurrentDeferredRevenue,trailingCurrentAccruedExpenses,trailingIncomeTaxPayable,trailingAccountsPayable,trailingCurrentDebt,trailingTotalNonCurrentAssets,trailingOtherNonCurrentAssets,trailingOtherIntangibleAssets,trailingGoodwill,trailingInvestmentsAndAdvances,trailingNetPPE,trailingAccumulatedDepreciation,trailingGrossPPE,trailingCurrentAssets,trailingOtherCurrentAssets,trailingInventory,trailingAccountsReceivable,trailingCashCashEquivalentsAndShortTermInvestments,trailingOtherShortTermInvestments,trailingCashAndCashEquivalents",
            });
            query.append_pair("merge", "false");
            query.append_pair("period1", "0");
            query.append_pair("period2", &ts.to_string());
        }
        let mut value = send_request(&url.to_string()).await?;
        let mut value = value["timeseries"].take()["result"].take();
        let data = value.as_array().unwrap();
        let mut fields = HashMap::new();
        for value in data {
            let key = value["meta"]["type"].as_array().unwrap().first().and_then(|v| v.as_str()).unwrap();
            let timestamp = value["timestamp"].as_array().cloned().unwrap_or_default().into_iter().map(|v| v.as_i64().unwrap()).map(|i| OffsetDateTime::from_unix_timestamp(i).unwrap());
            let value = value[key].as_array().cloned().unwrap_or_default().into_iter().map(|v| v["reportedValue"]["raw"].as_f64().unwrap());
            fields.insert(key.to_string(), timestamp.zip(value).collect());
        }
        Ok(fields)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FinancialsPeriod {
    Quarterly,
    Annual,
    Trailing
}

/// Send request to yahoo! finance server and transform response to JSON value
async fn send_request(url: &str) -> Result<serde_json::Value, YahooError> {
    let resp = reqwest::get(url).compat().await;
    if resp.is_err() {
        return Err(YahooError::ConnectionFailed);
    }
    let resp = resp.unwrap();
    match resp.status() {
        StatusCode::OK => resp.json().await.map_err(|_| YahooError::InvalidJson),
        status => Err(YahooError::FetchFailed(format!("Status Code: {}", status))),
    }
}

/// Send request to yahoo! finance server and transform response to JSON value
async fn send_scrape_request(url: &str) -> Result<serde_json::Value, YahooError> {
    let resp = reqwest::get(url).compat().await;
    if resp.is_err() {
        return Err(YahooError::ConnectionFailed);
    }
    let resp = resp.unwrap();
    let status = resp.status();
    let html_text = resp.text().await.unwrap();
    let json_str = html_text.split("root.App.main = ").nth(1)
        .and_then(|o| o.split("(this)").next())
        .and_then(|o| o.split(";\n").next())
        .map(|o| o.trim());
    match status {
        StatusCode::OK => if let Some(json_str) = json_str {
            let json = serde_json::from_str::<serde_json::Value>(json_str).unwrap();
            let stores = json.get("context")
                .and_then(|json| json.get("dispatcher"))
                .and_then(|json| json.get("stores"))
                .unwrap()
                .clone();
            // println!("{}", stores);
            serde_json::from_value(stores).map_err(|_| YahooError::InvalidJson)
        } else {
            Err(YahooError::FetchFailed("failed to find json in html".to_string()))
        },
        status => Err(YahooError::FetchFailed(format!("Status Code: {}", status))),
    }
    
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use super::*;

    #[test]
    fn test_get_single_quote() {
        let provider = YahooConnector::new();
        let response = tokio_test::block_on(provider.get_latest_quotes("HNL.DE", "1d")).unwrap();
        assert_eq!(&response.chart.result[0].meta.symbol, "HNL.DE");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_get_financials() {
        let provider = YahooConnector::new();
        for target in ["HNL.DE", "LCY.AX", "NFL.AX"] {
            let response = tokio_test::block_on(provider.get_financials(target)).unwrap();
            println!("{response:?}");
            // assert_ne!(response.shares_on_issue(), None);
        }
    }

    #[test]
    fn test_get_statistics() {
        let provider = YahooConnector::new();
        for target in ["LEL.AX", "NFL.AX"] {
            let response = tokio_test::block_on(provider.get_statistics(target)).unwrap();
            println!("{response:?}");
            // assert_ne!(response.shares_on_issue(), None);
        }
    }

    #[test]
    fn test_strange_api_responses() {
        let provider = YahooConnector::new();
        let start = datetime!(2019-7-3 00:00 UTC); // Utc.ymd(2019, 7, 3).and_hms_milli(0, 0, 0, 0);
        let end = datetime!(2020-7-4 23:59:59.999 UTC); // Utc.ymd(2020, 7, 4).and_hms_milli(23, 59, 59, 999);
        let resp = tokio_test::block_on(provider.get_quote_history("IBM", start, end)).unwrap();

        assert_eq!(&resp.chart.result[0].meta.symbol, "IBM");
        assert_eq!(&resp.chart.result[0].meta.data_granularity, "1d");
        assert_eq!(&resp.chart.result[0].meta.first_trade_date, &-252322200);

        let _ = resp.last_quote().unwrap();
    }

    #[test]
    #[should_panic(expected = "DeserializeFailed(\"missing field `adjclose`\")")]
    fn test_api_responses_missing_fields() {
        let provider = YahooConnector::new();
        let response = tokio_test::block_on(provider.get_latest_quotes("BF.B", "1m")).unwrap();

        assert_eq!(&response.chart.result[0].meta.symbol, "BF.B");
        assert_eq!(&response.chart.result[0].meta.range, "1d");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1m");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_get_quote_history() {
        let provider = YahooConnector::new();
        let start = datetime!(2020-1-1 00:00 UTC);
        let end = datetime!(2020-1-31 23:59:59.999 UTC);
        let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end));
        if resp.is_ok() {
            let resp = resp.unwrap();
            assert_eq!(resp.chart.result[0].timestamp.len(), 21);
            let quotes = resp.quotes().unwrap();
            assert_eq!(quotes.len(), 21);
        }
    }

    #[test]
    fn test_get_quote_range() {
        let provider = YahooConnector::new();
        let response =
            tokio_test::block_on(provider.get_quote_range("HNL.DE", "1d", "1mo")).unwrap();
        assert_eq!(&response.chart.result[0].meta.symbol, "HNL.DE");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let _ = response.last_quote().unwrap();
    }

    #[test]
    fn test_get() {
        let provider = YahooConnector::new();
        let start = datetime!(2019-1-1 00:00 UTC);
        let end = datetime!(2020-1-31 23:59:59.999 UTC);
        let response =
            tokio_test::block_on(provider.get_quote_history_interval("AAPL", start, end, "1mo"))
                .unwrap();
        assert_eq!(&response.chart.result[0].timestamp.len(), &13);
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1mo");
        let quotes = response.quotes().unwrap();
        assert_eq!(quotes.len(), 13usize);
    }

    #[test]
    fn test_large_volume() {
        let provider = YahooConnector::new();
        let response =
            tokio_test::block_on(provider.get_quote_range("BTC-USD", "1d", "5d")).unwrap();
        let quotes = response.quotes().unwrap();
        assert!(quotes.len() > 0usize);
    }

    #[test]
    fn test_search_ticker() {
        let provider = YahooConnector::new();
        let resp = tokio_test::block_on(provider.search_ticker("Apple")).unwrap();

        assert_eq!(resp.count, 15);
        let mut apple_found = false;
        for item in resp.quotes {
            if item.exchange == "NMS" && item.symbol == "AAPL" && item.short_name == "Apple Inc." {
                apple_found = true;
                break;
            }
        }
        assert!(apple_found)
    }

    #[test]
    fn test_mutual_fund_history() {
        let provider = YahooConnector::new();
        let start = datetime!(2020-1-1 00:00 UTC);
        let end = datetime!(2020-1-31 23:59:59.999 UTC);
        let resp = tokio_test::block_on(provider.get_quote_history("VTSAX", start, end));
        if resp.is_ok() {
            let resp = resp.unwrap();
            assert_eq!(resp.chart.result[0].timestamp.len(), 21);
            let quotes = resp.quotes().unwrap();
            assert_eq!(quotes.len(), 21);
            println!("{:?}", quotes);
        }
    }

    #[test]
    fn test_mutual_fund_latest() {
        let provider = YahooConnector::new();
        let response = tokio_test::block_on(provider.get_latest_quotes("VTSAX", "1d")).unwrap();

        assert_eq!(&response.chart.result[0].meta.symbol, "VTSAX");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
        let _ = response.last_quote().unwrap();
    }

    
    #[test]
    fn test_mutual_fund_range() {
        let provider = YahooConnector::new();
        let response =
            tokio_test::block_on(provider.get_quote_range("VTSAX", "1d", "1mo")).unwrap();
        assert_eq!(&response.chart.result[0].meta.symbol, "VTSAX");
        assert_eq!(&response.chart.result[0].meta.range, "1mo");
        assert_eq!(&response.chart.result[0].meta.data_granularity, "1d");
    }

    
    #[test]
    fn test_get_financials_timeseries() {
        let provider = YahooConnector::new();
        let response = tokio_test::block_on(provider.get_financials_timeseries("NFL.AX", FinancialsPeriod::Annual)).unwrap();
        println!("{response:#?}");
        let response = tokio_test::block_on(provider.get_financials_timeseries("NFL.AX", FinancialsPeriod::Quarterly)).unwrap();
    }
}
