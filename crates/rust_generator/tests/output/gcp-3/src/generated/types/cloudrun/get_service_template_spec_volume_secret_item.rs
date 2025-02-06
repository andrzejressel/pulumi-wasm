#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecVolumeSecretItem {
    /// The Cloud Secret Manager secret version.
    /// Can be 'latest' for the latest value or an integer for a specific version.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Mode bits to use on this file, must be a value between 0000 and 0777. If
    /// not specified, the volume defaultMode will be used. This might be in
    /// conflict with other options that affect the file mode, like fsGroup, and
    /// the result can be other mode bits set.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<i32>,
    /// The relative path of the file to map the key to.
    /// May not be an absolute path.
    /// May not contain the path element '..'.
    /// May not start with the string '..'.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
