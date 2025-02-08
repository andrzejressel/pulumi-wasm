#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IngestionDestinationDestinationConfigurationAuditLog {
    /// Contains information about an audit log destination. Only one destination (Firehose Stream) or (S3 Bucket) can be specified.
    #[builder(into, default)]
    #[serde(rename = "destination")]
    pub r#destination: Box<Option<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestination>>,
}
