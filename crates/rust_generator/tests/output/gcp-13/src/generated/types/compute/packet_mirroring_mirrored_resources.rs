#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PacketMirroringMirroredResources {
    /// All the listed instances will be mirrored.  Specify at most 50.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "instances")]
    pub r#instances: Box<Option<Vec<super::super::types::compute::PacketMirroringMirroredResourcesInstance>>>,
    /// All instances in one of these subnetworks will be mirrored.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subnetworks")]
    pub r#subnetworks: Box<Option<Vec<super::super::types::compute::PacketMirroringMirroredResourcesSubnetwork>>>,
    /// All instances with these tags will be mirrored.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
}
