#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigMfaProviderConfigTotpProviderConfig {
    /// The allowed number of adjacent intervals that will be used for verification to avoid clock skew.
    #[builder(into, default)]
    #[serde(rename = "adjacentIntervals")]
    pub r#adjacent_intervals: Box<Option<i32>>,
}
