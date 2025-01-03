#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IngestionDestinationProcessingConfiguration {
    /// Contains information about an audit log processing configuration.
    #[builder(into, default)]
    #[serde(rename = "auditLog")]
    pub r#audit_log: Box<Option<super::super::types::appfabric::IngestionDestinationProcessingConfigurationAuditLog>>,
}
