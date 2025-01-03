#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyAdmissionWhitelistPattern {
    /// An image name pattern to whitelist, in the form
    /// `registry/path/to/image`. This supports a trailing * as a
    /// wildcard, but this is allowed only in text after the registry/
    /// part.
    #[builder(into)]
    #[serde(rename = "namePattern")]
    pub r#name_pattern: Box<String>,
}
