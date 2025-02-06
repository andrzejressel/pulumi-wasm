#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StackStreamingExperienceSettings {
    /// The preferred protocol that you want to use while streaming your application.
    /// Valid values are `TCP` and `UDP`.
    #[builder(into, default)]
    #[serde(rename = "preferredProtocol")]
    pub r#preferred_protocol: Box<Option<String>>,
}
