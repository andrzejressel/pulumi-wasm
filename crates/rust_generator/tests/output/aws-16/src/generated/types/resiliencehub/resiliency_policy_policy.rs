#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResiliencyPolicyPolicy {
    /// Specifies Availability Zone failure policy. See `policy.az`
    #[builder(into, default)]
    #[serde(rename = "az")]
    pub r#az: Box<Option<super::super::types::resiliencehub::ResiliencyPolicyPolicyAz>>,
    /// Specifies Infrastructure failure policy. See `policy.hardware`
    #[builder(into, default)]
    #[serde(rename = "hardware")]
    pub r#hardware: Box<Option<super::super::types::resiliencehub::ResiliencyPolicyPolicyHardware>>,
    /// Specifies Region failure policy. `policy.region`
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<super::super::types::resiliencehub::ResiliencyPolicyPolicyRegion>>,
    /// Specifies Application failure policy. See `policy.software`
    /// 
    /// The following arguments are optional:
    #[builder(into, default)]
    #[serde(rename = "software")]
    pub r#software: Box<Option<super::super::types::resiliencehub::ResiliencyPolicyPolicySoftware>>,
}
