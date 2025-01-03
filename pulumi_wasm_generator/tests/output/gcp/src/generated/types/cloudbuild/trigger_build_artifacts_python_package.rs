#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerBuildArtifactsPythonPackage {
    /// Path globs used to match files in the build's workspace. For Python/ Twine, this is usually dist/*, and sometimes additionally an .asc file.
    #[builder(into, default)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Option<Vec<String>>>,
    /// Artifact Registry repository, in the form "https://$REGION-python.pkg.dev/$PROJECT/$REPOSITORY"
    /// Files in the workspace matching any path pattern will be uploaded to Artifact Registry with this location as a prefix.
    #[builder(into, default)]
    #[serde(rename = "repository")]
    pub r#repository: Box<Option<String>>,
}
