#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationInstallPatchWindow {
    /// List of Classification category of patches to be patched.
    #[builder(into)]
    #[serde(rename = "classificationsToIncludes")]
    pub r#classifications_to_includes: Box<Vec<String>>,
    /// List of KB numbers to be excluded from patching.
    #[builder(into)]
    #[serde(rename = "kbNumbersToExcludes")]
    pub r#kb_numbers_to_excludes: Box<Vec<String>>,
    /// List of KB numbers to be included for patching.
    #[builder(into)]
    #[serde(rename = "kbNumbersToIncludes")]
    pub r#kb_numbers_to_includes: Box<Vec<String>>,
}
