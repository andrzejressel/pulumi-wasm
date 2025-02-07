#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFunctionServiceConfigSecretVolume {
    /// The path within the container to mount the secret volume. For example, setting the mountPath as /etc/secrets would mount the secret value files under the /etc/secrets directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount path: /etc/secrets
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<String>,
    /// Project identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
    /// Name of the secret in secret manager (not the full resource name).
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
    /// List of secret versions to mount for this secret. If empty, the latest version of the secret will be made available in a file named after the secret under the mount point.'
    #[builder(into)]
    #[serde(rename = "versions")]
    pub r#versions: Box<Vec<super::super::types::cloudfunctionsv2::GetFunctionServiceConfigSecretVolumeVersion>>,
}
