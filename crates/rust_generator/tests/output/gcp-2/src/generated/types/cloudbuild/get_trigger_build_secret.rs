#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerBuildSecret {
    /// Cloud KMS key name to use to decrypt these envs.
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<String>,
    /// Map of environment variable name to its encrypted value.
    /// Secret environment variables must be unique across all of a build's secrets,
    /// and must be used by at least one build step. Values can be at most 64 KB in size.
    /// There can be at most 100 secret values across all of a build's secrets.
    #[builder(into)]
    #[serde(rename = "secretEnv")]
    pub r#secret_env: Box<std::collections::HashMap<String, String>>,
}
