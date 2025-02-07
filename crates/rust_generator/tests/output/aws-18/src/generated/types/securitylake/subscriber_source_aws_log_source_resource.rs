#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriberSourceAwsLogSourceResource {
    /// Provides data expiration details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Box<String>,
    /// Provides data storage transition details of Amazon Security Lake object.
    #[builder(into, default)]
    #[serde(rename = "sourceVersion")]
    pub r#source_version: Box<Option<String>>,
}
