#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeSourceParametersSelfManagedKafkaParametersVpc {
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Option<Vec<String>>>,
}
