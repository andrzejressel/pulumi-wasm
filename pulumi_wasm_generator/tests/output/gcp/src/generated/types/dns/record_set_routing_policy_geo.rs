#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecordSetRoutingPolicyGeo {
    /// For A and AAAA types only. The list of targets to be health checked. These can be specified along with `rrdatas` within this item.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "healthCheckedTargets")]
    pub r#health_checked_targets: Box<Option<super::super::types::dns::RecordSetRoutingPolicyGeoHealthCheckedTargets>>,
    /// The location name defined in Google Cloud.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Same as `rrdatas` above.
    #[builder(into, default)]
    #[serde(rename = "rrdatas")]
    pub r#rrdatas: Box<Option<Vec<String>>>,
}
