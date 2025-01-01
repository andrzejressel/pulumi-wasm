#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerBuildAvailableSecretsSecretManager {
    /// Environment variable name to associate with the secret. Secret environment
    /// variables must be unique across all of a build's secrets, and must be used
    /// by at least one build step.
    #[builder(into)]
    #[serde(rename = "env")]
    pub r#env: Box<String>,
    /// Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/*
    #[builder(into)]
    #[serde(rename = "versionName")]
    pub r#version_name: Box<String>,
}
