#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccessPointVpcConfiguration {
    /// This access point will only allow connections from the specified VPC ID.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
