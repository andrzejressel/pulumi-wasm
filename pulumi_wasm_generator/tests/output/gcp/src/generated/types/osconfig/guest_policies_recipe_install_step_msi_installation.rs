#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GuestPoliciesRecipeInstallStepMsiInstallation {
    /// Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]
    #[builder(into, default)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Box<Option<Vec<i32>>>,
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Box<String>,
    /// The flags to use when installing the MSI. Defaults to the install flag.
    #[builder(into, default)]
    #[serde(rename = "flags")]
    pub r#flags: Box<Option<Vec<String>>>,
}
