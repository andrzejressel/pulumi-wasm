#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationAttributes {
    /// Optional. Business team that ensures user needs are met and value is delivered
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "businessOwners")]
    pub r#business_owners: Box<Option<Vec<super::super::types::apphub::ApplicationAttributesBusinessOwner>>>,
    /// Criticality of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "criticality")]
    pub r#criticality: Box<Option<super::super::types::apphub::ApplicationAttributesCriticality>>,
    /// Optional. Developer team that owns development and coding.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "developerOwners")]
    pub r#developer_owners: Box<Option<Vec<super::super::types::apphub::ApplicationAttributesDeveloperOwner>>>,
    /// Environment of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<super::super::types::apphub::ApplicationAttributesEnvironment>>,
    /// Optional. Operator team that ensures runtime and operations.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "operatorOwners")]
    pub r#operator_owners: Box<Option<Vec<super::super::types::apphub::ApplicationAttributesOperatorOwner>>>,
}
