#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostingCustomDomainCertVerificationHttp {
    /// A text string to serve at the path.
    #[builder(into, default)]
    #[serde(rename = "desired")]
    pub r#desired: Box<Option<String>>,
    /// Whether Hosting was able to find the required file contents on the
    /// specified path during its last check.
    #[builder(into, default)]
    #[serde(rename = "discovered")]
    pub r#discovered: Box<Option<String>>,
    /// (Output)
    /// The last time Hosting systems checked for the file contents.
    #[builder(into, default)]
    #[serde(rename = "lastCheckTime")]
    pub r#last_check_time: Box<Option<String>>,
    /// The path to the file.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
}
