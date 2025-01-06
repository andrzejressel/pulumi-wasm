#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeGroupSapHanaVolumeDataProtectionReplication {
    /// The endpoint type. Possible values are `dst` and `src`. Defaults to `dst`.
    #[builder(into, default)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: Box<Option<String>>,
    /// Location of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeLocation")]
    pub r#remote_volume_location: Box<String>,
    /// Resource ID of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeResourceId")]
    pub r#remote_volume_resource_id: Box<String>,
    /// eplication frequency. Possible values are `10minutes`, `daily` and `hourly`.
    #[builder(into)]
    #[serde(rename = "replicationFrequency")]
    pub r#replication_frequency: Box<String>,
}
