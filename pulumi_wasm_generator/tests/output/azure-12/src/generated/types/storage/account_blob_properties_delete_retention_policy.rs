#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountBlobPropertiesDeleteRetentionPolicy {
    /// Specifies the number of days that the blob should be retained, between `1` and `365` days. Defaults to `7`.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
    /// Indicates whether permanent deletion of the soft deleted blob versions and snapshots is allowed. Defaults to `false`.
    /// 
    /// > **Note:** `permanent_delete_enabled` cannot be set to true if a `restore_policy` block is defined.
    #[builder(into, default)]
    #[serde(rename = "permanentDeleteEnabled")]
    pub r#permanent_delete_enabled: Box<Option<bool>>,
}
