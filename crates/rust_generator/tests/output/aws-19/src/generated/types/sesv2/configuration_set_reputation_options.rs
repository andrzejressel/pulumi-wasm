#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationSetReputationOptions {
    /// The date and time (in Unix time) when the reputation metrics were last given a fresh start. When your account is given a fresh start, your reputation metrics are calculated starting from the date of the fresh start.
    #[builder(into, default)]
    #[serde(rename = "lastFreshStart")]
    pub r#last_fresh_start: Box<Option<String>>,
    /// If `true`, tracking of reputation metrics is enabled for the configuration set. If `false`, tracking of reputation metrics is disabled for the configuration set.
    #[builder(into, default)]
    #[serde(rename = "reputationMetricsEnabled")]
    pub r#reputation_metrics_enabled: Box<Option<bool>>,
}
