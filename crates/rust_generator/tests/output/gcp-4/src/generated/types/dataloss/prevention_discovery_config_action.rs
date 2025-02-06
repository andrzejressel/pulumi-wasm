#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigAction {
    /// Export data profiles into a provided location
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "exportData")]
    pub r#export_data: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigActionExportData>>,
    /// Publish a message into the Pub/Sub topic.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pubSubNotification")]
    pub r#pub_sub_notification: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigActionPubSubNotification>>,
    /// Publish a message into the Pub/Sub topic.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "tagResources")]
    pub r#tag_resources: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigActionTagResources>>,
}
