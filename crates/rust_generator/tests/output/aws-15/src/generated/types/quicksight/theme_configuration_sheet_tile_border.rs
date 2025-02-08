#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ThemeConfigurationSheetTileBorder {
    /// The option to enable display of borders for visuals.
    #[builder(into, default)]
    #[serde(rename = "show")]
    pub r#show: Box<Option<bool>>,
}
