#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetastoreServiceHiveMetastoreConfigKerberosConfigKeytab {
    /// The relative resource name of a Secret Manager secret version, in the following form:
    /// "projects/{projectNumber}/secrets/{secret_id}/versions/{version_id}".
    #[builder(into)]
    #[serde(rename = "cloudSecret")]
    pub r#cloud_secret: Box<String>,
}
