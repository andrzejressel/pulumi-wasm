#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceAttributes {
    /// Business team that ensures user needs are met and value is delivered
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "businessOwners")]
    pub r#business_owners: Box<Option<Vec<super::super::types::apphub::ServiceAttributesBusinessOwner>>>,
    /// Criticality of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "criticality")]
    pub r#criticality: Box<Option<super::super::types::apphub::ServiceAttributesCriticality>>,
    /// Developer team that owns development and coding.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "developerOwners")]
    pub r#developer_owners: Box<Option<Vec<super::super::types::apphub::ServiceAttributesDeveloperOwner>>>,
    /// Environment of the Application, Service, or Workload
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<super::super::types::apphub::ServiceAttributesEnvironment>>,
    /// Operator team that ensures runtime and operations.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "operatorOwners")]
    pub r#operator_owners: Box<Option<Vec<super::super::types::apphub::ServiceAttributesOperatorOwner>>>,
}
