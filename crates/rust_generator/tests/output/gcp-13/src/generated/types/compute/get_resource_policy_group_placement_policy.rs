#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourcePolicyGroupPlacementPolicy {
    /// The number of availability domains instances will be spread across. If two instances are in different
    /// availability domain, they will not be put in the same low latency network
    #[builder(into)]
    #[serde(rename = "availabilityDomainCount")]
    pub r#availability_domain_count: Box<i32>,
    /// Collocation specifies whether to place VMs inside the same availability domain on the same low-latency network.
    /// Specify 'COLLOCATED' to enable collocation. Can only be specified with 'vm_count'. If compute instances are created
    /// with a COLLOCATED policy, then exactly 'vm_count' instances must be created at the same time with the resource policy
    /// attached. Possible values: ["COLLOCATED"]
    #[builder(into)]
    #[serde(rename = "collocation")]
    pub r#collocation: Box<String>,
    /// Specifies the number of max logical switches.
    #[builder(into)]
    #[serde(rename = "maxDistance")]
    pub r#max_distance: Box<i32>,
    /// Number of VMs in this placement group. Google does not recommend that you use this field
    /// unless you use a compact policy and you want your policy to work only if it contains this
    /// exact number of VMs.
    #[builder(into)]
    #[serde(rename = "vmCount")]
    pub r#vm_count: Box<i32>,
}
