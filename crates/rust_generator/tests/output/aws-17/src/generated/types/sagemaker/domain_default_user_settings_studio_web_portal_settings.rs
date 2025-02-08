#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultUserSettingsStudioWebPortalSettings {
    /// The Applications supported in Studio that are hidden from the Studio left navigation pane.
    #[builder(into, default)]
    #[serde(rename = "hiddenAppTypes")]
    pub r#hidden_app_types: Box<Option<Vec<String>>>,
    /// The instance types you are hiding from the Studio user interface.
    #[builder(into, default)]
    #[serde(rename = "hiddenInstanceTypes")]
    pub r#hidden_instance_types: Box<Option<Vec<String>>>,
    /// The machine learning tools that are hidden from the Studio left navigation pane.
    #[builder(into, default)]
    #[serde(rename = "hiddenMlTools")]
    pub r#hidden_ml_tools: Box<Option<Vec<String>>>,
}
