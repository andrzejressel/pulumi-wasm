#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetLaunchTemplateConfigOverrideInstanceRequirementsBaselineEbsBandwidthMbps {
    /// The maximum baseline bandwidth, in Mbps. To specify no maximum limit, omit this parameter..
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<i32>>,
    /// The minimum baseline bandwidth, in Mbps. To specify no minimum limit, omit this parameter..
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<i32>>,
}
