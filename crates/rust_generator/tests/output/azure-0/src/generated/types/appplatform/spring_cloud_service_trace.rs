#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudServiceTrace {
    /// The connection string used for Application Insights.
    #[builder(into, default)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<Option<String>>,
    /// The sampling rate of Application Insights Agent. Must be between `0.0` and `100.0`. Defaults to `10.0`.
    #[builder(into, default)]
    #[serde(rename = "sampleRate")]
    pub r#sample_rate: Box<Option<f64>>,
}
