#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateSpecVolumeNfs {
    /// Path exported by the NFS server
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// If true, mount the NFS volume as read only in all mounts. Defaults to false.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// IP address or hostname of the NFS server
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: Box<String>,
}
