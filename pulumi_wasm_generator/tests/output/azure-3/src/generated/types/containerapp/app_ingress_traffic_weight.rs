#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppIngressTrafficWeight {
    /// The label to apply to the revision as a name prefix for routing traffic.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// This traffic Weight applies to the latest stable Container Revision. At most only one `traffic_weight` block can have the `latest_revision` set to `true`.
    #[builder(into, default)]
    #[serde(rename = "latestRevision")]
    pub r#latest_revision: Box<Option<bool>>,
    /// The percentage of traffic which should be sent this revision.
    /// 
    /// > **Note:** The cumulative values for `weight` must equal 100 exactly and explicitly, no default weights are assumed.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<i32>,
    /// The suffix string to which this `traffic_weight` applies.
    /// 
    /// > **Note:** If `latest_revision` is `false`, the `revision_suffix` shall be specified.
    #[builder(into, default)]
    #[serde(rename = "revisionSuffix")]
    pub r#revision_suffix: Box<Option<String>>,
}
