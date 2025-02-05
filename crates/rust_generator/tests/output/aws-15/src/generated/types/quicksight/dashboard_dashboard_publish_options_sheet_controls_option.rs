#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DashboardDashboardPublishOptionsSheetControlsOption {
    /// Visibility state. Possibles values: EXPANDED, COLLAPSED.
    #[builder(into, default)]
    #[serde(rename = "visibilityState")]
    pub r#visibility_state: Box<Option<String>>,
}
