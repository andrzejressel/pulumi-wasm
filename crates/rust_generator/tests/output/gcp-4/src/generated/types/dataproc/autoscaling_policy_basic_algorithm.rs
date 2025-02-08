#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscalingPolicyBasicAlgorithm {
    /// Duration between scaling events. A scaling period starts after the
    /// update operation from the previous event has completed.
    /// Bounds: [2m, 1d]. Default: 2m.
    #[builder(into, default)]
    #[serde(rename = "cooldownPeriod")]
    pub r#cooldown_period: Box<Option<String>>,
    /// YARN autoscaling configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "yarnConfig")]
    pub r#yarn_config: Box<super::super::types::dataproc::AutoscalingPolicyBasicAlgorithmYarnConfig>,
}
