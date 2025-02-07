#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationAppSource {
    /// Password to use when authenticating to the source. This provider cannot perform drift detection of this configuration.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// For sources that are version-aware, the revision to use.
    #[builder(into, default)]
    #[serde(rename = "revision")]
    pub r#revision: Box<Option<String>>,
    /// SSH key to use when authenticating to the source. This provider cannot perform drift detection of this configuration.
    #[builder(into, default)]
    #[serde(rename = "sshKey")]
    pub r#ssh_key: Box<Option<String>>,
    /// The type of source to use. For example, "archive".
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The URL where the app resource can be found.
    #[builder(into, default)]
    #[serde(rename = "url")]
    pub r#url: Box<Option<String>>,
    /// Username to use when authenticating to the source.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
