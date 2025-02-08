#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ZoneVpc {
    /// ID of the VPC to associate.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
    /// Region of the VPC to associate. Defaults to AWS provider region.
    #[builder(into, default)]
    #[serde(rename = "vpcRegion")]
    pub r#vpc_region: Box<Option<String>>,
}
