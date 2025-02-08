#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RecordSetRoutingPolicyPrimaryBackupBackupGeo {
    /// For A and AAAA types only. The list of targets to be health checked. These can be specified along with `rrdatas` within this item.
    #[builder(into, default)]
    #[serde(rename = "healthCheckedTargets")]
    pub r#health_checked_targets: Box<Option<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupBackupGeoHealthCheckedTargets>>,
    /// The location name defined in Google Cloud.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "rrdatas")]
    pub r#rrdatas: Box<Option<Vec<String>>>,
}
