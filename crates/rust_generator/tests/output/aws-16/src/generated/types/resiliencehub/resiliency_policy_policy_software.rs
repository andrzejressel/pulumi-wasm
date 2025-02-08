#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResiliencyPolicyPolicySoftware {
    /// Recovery Point Objective (RPO) as a Go duration.
    #[builder(into)]
    #[serde(rename = "rpo")]
    pub r#rpo: Box<String>,
    /// Recovery Time Objective (RTO) as a Go duration.
    #[builder(into)]
    #[serde(rename = "rto")]
    pub r#rto: Box<String>,
}
