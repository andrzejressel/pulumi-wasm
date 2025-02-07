#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureResourceState {
    /// (Output)
    /// Whether this Feature has outstanding resources that need to be cleaned up before it can be disabled.
    #[builder(into, default)]
    #[serde(rename = "hasResources")]
    pub r#has_resources: Box<Option<bool>>,
    /// (Output)
    /// Output only. The "running state" of the Feature in this Hub.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
