#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationAttribute {
    /// Optional. Business team that ensures user needs are met and value is delivered
    #[builder(into)]
    #[serde(rename = "businessOwners")]
    pub r#business_owners: Box<Vec<super::super::types::apphub::GetApplicationAttributeBusinessOwner>>,
    /// Criticality of the Application, Service, or Workload
    #[builder(into)]
    #[serde(rename = "criticalities")]
    pub r#criticalities: Box<Vec<super::super::types::apphub::GetApplicationAttributeCriticality>>,
    /// Optional. Developer team that owns development and coding.
    #[builder(into)]
    #[serde(rename = "developerOwners")]
    pub r#developer_owners: Box<Vec<super::super::types::apphub::GetApplicationAttributeDeveloperOwner>>,
    /// Environment of the Application, Service, or Workload
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Box<Vec<super::super::types::apphub::GetApplicationAttributeEnvironment>>,
    /// Optional. Operator team that ensures runtime and operations.
    #[builder(into)]
    #[serde(rename = "operatorOwners")]
    pub r#operator_owners: Box<Vec<super::super::types::apphub::GetApplicationAttributeOperatorOwner>>,
}
