#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetThemeConfigurationSheetTileLayoutMargin {
    /// This Boolean value controls whether to display sheet margins.
    #[builder(into)]
    #[serde(rename = "show")]
    pub r#show: Box<bool>,
}