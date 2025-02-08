#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowSourceFlowConfigSourceConnectorPropertiesSalesforce {
    /// Flag that enables dynamic fetching of new (recently added) fields in the Salesforce objects while running a flow.
    #[builder(into, default)]
    #[serde(rename = "enableDynamicFieldUpdate")]
    pub r#enable_dynamic_field_update: Box<Option<bool>>,
    /// Whether Amazon AppFlow includes deleted files in the flow run.
    #[builder(into, default)]
    #[serde(rename = "includeDeletedRecords")]
    pub r#include_deleted_records: Box<Option<bool>>,
    #[builder(into)]
    #[serde(rename = "object")]
    pub r#object: Box<String>,
}
