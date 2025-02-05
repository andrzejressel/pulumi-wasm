#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDataLakeSettingsCreateDatabaseDefaultPermission {
    /// List of permissions granted to the principal.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Vec<String>>,
    /// Principal who is granted permissions.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
