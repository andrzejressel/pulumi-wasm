#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountBlobPropertiesRestorePolicy {
    /// Specifies the number of days that the blob can be restored, between `1` and `365` days. This must be less than the `days` specified for `delete_retention_policy`.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Box<i32>,
}
