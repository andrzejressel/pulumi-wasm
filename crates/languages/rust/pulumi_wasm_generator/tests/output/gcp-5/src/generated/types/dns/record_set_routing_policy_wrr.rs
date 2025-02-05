#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecordSetRoutingPolicyWrr {
    /// The list of targets to be health checked. Note that if DNSSEC is enabled for this zone, only one of `rrdatas` or `health_checked_targets` can be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "healthCheckedTargets")]
    pub r#health_checked_targets: Box<Option<super::super::types::dns::RecordSetRoutingPolicyWrrHealthCheckedTargets>>,
    /// Same as `rrdatas` above.
    #[builder(into, default)]
    #[serde(rename = "rrdatas")]
    pub r#rrdatas: Box<Option<Vec<String>>>,
    /// The ratio of traffic routed to the target.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<f64>,
}
