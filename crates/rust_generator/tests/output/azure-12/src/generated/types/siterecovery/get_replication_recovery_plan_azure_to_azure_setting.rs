#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetReplicationRecoveryPlanAzureToAzureSetting {
    #[builder(into)]
    #[serde(rename = "primaryEdgeZone")]
    pub r#primary_edge_zone: Box<String>,
    #[builder(into)]
    #[serde(rename = "primaryZone")]
    pub r#primary_zone: Box<String>,
    #[builder(into)]
    #[serde(rename = "recoveryEdgeZone")]
    pub r#recovery_edge_zone: Box<String>,
    #[builder(into)]
    #[serde(rename = "recoveryZone")]
    pub r#recovery_zone: Box<String>,
}
