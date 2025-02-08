#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourcePolicyGroupPlacementPolicy {
    /// The number of availability domains instances will be spread across. If two instances are in different
    /// availability domain, they will not be put in the same low latency network
    #[builder(into, default)]
    #[serde(rename = "availabilityDomainCount")]
    pub r#availability_domain_count: Box<Option<i32>>,
    /// Collocation specifies whether to place VMs inside the same availability domain on the same low-latency network.
    /// Specify `COLLOCATED` to enable collocation. Can only be specified with `vm_count`. If compute instances are created
    /// with a COLLOCATED policy, then exactly `vm_count` instances must be created at the same time with the resource policy
    /// attached.
    /// Possible values are: `COLLOCATED`.
    #[builder(into, default)]
    #[serde(rename = "collocation")]
    pub r#collocation: Box<Option<String>>,
    /// Specifies the number of max logical switches.
    #[builder(into, default)]
    #[serde(rename = "maxDistance")]
    pub r#max_distance: Box<Option<i32>>,
    /// Number of VMs in this placement group. Google does not recommend that you use this field
    /// unless you use a compact policy and you want your policy to work only if it contains this
    /// exact number of VMs.
    #[builder(into, default)]
    #[serde(rename = "vmCount")]
    pub r#vm_count: Box<Option<i32>>,
}
