#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPolicyConditionConditionThresholdForecastOptions {
    /// The length of time into the future to forecast
    /// whether a timeseries will violate the threshold.
    /// If the predicted value is found to violate the
    /// threshold, and the violation is observed in all
    /// forecasts made for the Configured `duration`,
    /// then the timeseries is considered to be failing.
    #[builder(into)]
    #[serde(rename = "forecastHorizon")]
    pub r#forecast_horizon: Box<String>,
}
