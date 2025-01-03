#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateContainerVolumeMount {
    /// Path within the container at which the volume should be mounted. Must not contain ':'. For Cloud SQL volumes, it can be left empty, or must otherwise be /cloudsql. All instances defined in the Volume will be available as /cloudsql/[instance]. For more information on Cloud SQL volumes, visit https://cloud.google.com/sql/docs/mysql/connect-run
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Box<String>,
    /// The name of the Cloud Run v2 Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
