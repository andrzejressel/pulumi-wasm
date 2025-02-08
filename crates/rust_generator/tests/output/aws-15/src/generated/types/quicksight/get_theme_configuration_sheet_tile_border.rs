#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetThemeConfigurationSheetTileBorder {
    /// This Boolean value controls whether to display sheet margins.
    #[builder(into)]
    #[serde(rename = "show")]
    pub r#show: Box<bool>,
}
