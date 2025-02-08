#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricAlertAction {
    /// The ID of the Action Group can be sourced from the `azure.monitoring.ActionGroup` resource
    #[builder(into)]
    #[serde(rename = "actionGroupId")]
    pub r#action_group_id: Box<String>,
    /// The map of custom string properties to include with the post operation. These data are appended to the webhook payload.
    #[builder(into, default)]
    #[serde(rename = "webhookProperties")]
    pub r#webhook_properties: Box<Option<std::collections::HashMap<String, String>>>,
}
