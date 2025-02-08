#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectedRegistryNotification {
    /// The action of the artifact that wants to be subscribed for the Connected Registry. Possible values are `push`, `delete` and `*` (i.e. any).
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// The digest of the artifact that wants to be subscribed for the Connected Registry.
    /// 
    /// > **NOTE:** One of either `tag` or `digest` can be specified.
    #[builder(into, default)]
    #[serde(rename = "digest")]
    pub r#digest: Box<Option<String>>,
    /// The name of the artifact that wants to be subscribed for the Connected Registry.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The tag of the artifact that wants to be subscribed for the Connected Registry.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
}
