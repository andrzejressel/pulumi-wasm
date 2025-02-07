#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriberSubscriberIdentity {
    /// The AWS Regions where Security Lake is automatically enabled.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Box<String>,
    /// Provides encryption details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
