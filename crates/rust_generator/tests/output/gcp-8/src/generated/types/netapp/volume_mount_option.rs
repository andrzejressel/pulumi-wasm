#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeMountOption {
    /// (Output)
    /// Export path of the volume.
    #[builder(into, default)]
    #[serde(rename = "export")]
    pub r#export: Box<Option<String>>,
    /// (Output)
    /// Full export path of the volume.
    /// Format for NFS volumes: `<export_ip>:/<shareName>`
    /// Format for SMB volumes: `\\\\netbios_prefix-four_random_hex_letters.domain_name\\shareName`
    #[builder(into, default)]
    #[serde(rename = "exportFull")]
    pub r#export_full: Box<Option<String>>,
    /// (Output)
    /// Human-readable mount instructions.
    #[builder(into, default)]
    #[serde(rename = "instructions")]
    pub r#instructions: Box<Option<String>>,
    /// (Output)
    /// Protocol to mount with.
    #[builder(into, default)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<Option<String>>,
}
