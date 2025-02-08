#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketReplicationConfigurationRuleDestinationAccessControlTranslation {
    /// The override value for the owner on replicated objects. Currently only `Destination` is supported.
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Box<String>,
}
