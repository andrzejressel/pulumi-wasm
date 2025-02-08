#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataCollectionRuleDataSourcesPerformanceCounter {
    /// Specifies a list of specifier names of the performance counters you want to collect. To get a list of performance counters on Windows, run the command `typeperf`. Please see [this document](https://learn.microsoft.com/en-us/azure/azure-monitor/agents/data-sources-performance-counters#configure-performance-counters) for more information.
    #[builder(into)]
    #[serde(rename = "counterSpecifiers")]
    pub r#counter_specifiers: Box<Vec<String>>,
    /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The number of seconds between consecutive counter measurements (samples). The value should be integer between `1` and `1800` inclusive. `sampling_frequency_in_seconds` must be equal to `60` seconds for counters collected with `Microsoft-InsightsMetrics` stream.
    #[builder(into)]
    #[serde(rename = "samplingFrequencyInSeconds")]
    pub r#sampling_frequency_in_seconds: Box<i32>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to. Possible values include but not limited to `Microsoft-InsightsMetrics`,and `Microsoft-Perf`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
