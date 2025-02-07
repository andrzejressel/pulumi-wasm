#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResiliencyPolicyPolicyRegion {
    /// Recovery Point Objective (RPO) as a Go duration.
    #[builder(into, default)]
    #[serde(rename = "rpo")]
    pub r#rpo: Box<Option<String>>,
    /// Recovery Time Objective (RTO) as a Go duration.
    #[builder(into, default)]
    #[serde(rename = "rto")]
    pub r#rto: Box<Option<String>>,
}
