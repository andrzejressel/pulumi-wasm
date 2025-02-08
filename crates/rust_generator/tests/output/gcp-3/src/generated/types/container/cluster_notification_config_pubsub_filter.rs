#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNotificationConfigPubsubFilter {
    /// Can be used to filter what notifications are sent. Accepted values are `UPGRADE_AVAILABLE_EVENT`, `UPGRADE_EVENT` and `SECURITY_BULLETIN_EVENT`. See [Filtering notifications](https://cloud.google.com/kubernetes-engine/docs/concepts/cluster-notifications#filtering) for more details.
    #[builder(into)]
    #[serde(rename = "eventTypes")]
    pub r#event_types: Box<Vec<String>>,
}
