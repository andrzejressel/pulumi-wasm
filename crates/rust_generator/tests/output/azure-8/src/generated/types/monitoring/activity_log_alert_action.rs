#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ActivityLogAlertAction {
    /// The ID of the Action Group can be sourced from the `azure.monitoring.ActionGroup` resource.
    #[builder(into)]
    #[serde(rename = "actionGroupId")]
    pub r#action_group_id: Box<String>,
    /// The map of custom string properties to include with the post operation. These data are appended to the webhook payload.
    #[builder(into, default)]
    #[serde(rename = "webhookProperties")]
    pub r#webhook_properties: Box<Option<std::collections::HashMap<String, String>>>,
}
