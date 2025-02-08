#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeDataProtectionReplication {
    /// The endpoint type, default value is `dst` for destination.
    #[builder(into, default)]
    #[serde(rename = "endpointType")]
    pub r#endpoint_type: Box<Option<String>>,
    /// Location of the primary volume. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "remoteVolumeLocation")]
    pub r#remote_volume_location: Box<String>,
    /// Resource ID of the primary volume.
    #[builder(into)]
    #[serde(rename = "remoteVolumeResourceId")]
    pub r#remote_volume_resource_id: Box<String>,
    /// Replication frequency, supported values are '10minutes', 'hourly', 'daily', values are case sensitive.
    /// 
    /// A full example of the `data_protection_replication` attribute can be found in the `./examples/netapp/volume_crr` directory within the GitHub Repository
    /// 
    /// > **NOTE:** `data_protection_replication` can be defined only once per secondary volume, adding a second instance of it is not supported.
    #[builder(into)]
    #[serde(rename = "replicationFrequency")]
    pub r#replication_frequency: Box<String>,
}
