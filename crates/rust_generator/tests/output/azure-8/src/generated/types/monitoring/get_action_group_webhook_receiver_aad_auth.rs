#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetActionGroupWebhookReceiverAadAuth {
    #[builder(into)]
    #[serde(rename = "identifierUri")]
    pub r#identifier_uri: Box<String>,
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<String>,
    /// The Tenant ID for the subscription containing this Event Hub.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
