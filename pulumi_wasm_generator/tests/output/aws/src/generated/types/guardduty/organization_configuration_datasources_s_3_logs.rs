#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrganizationConfigurationDatasourcesS3Logs {
    /// Set to `true` if you want S3 data event logs to be automatically enabled for new members of the organization. Default: `false`
    #[builder(into)]
    #[serde(rename = "autoEnable")]
    pub r#auto_enable: Box<bool>,
}