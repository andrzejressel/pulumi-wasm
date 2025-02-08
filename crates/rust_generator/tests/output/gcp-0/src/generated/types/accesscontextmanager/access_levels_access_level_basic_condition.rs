#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccessLevelsAccessLevelBasicCondition {
    /// Device specific restrictions, all restrictions must hold for
    /// the Condition to be true. If not specified, all devices are
    /// allowed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "devicePolicy")]
    pub r#device_policy: Box<Option<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasicConditionDevicePolicy>>,
    /// A list of CIDR block IP subnetwork specification. May be IPv4
    /// or IPv6.
    /// Note that for a CIDR IP address block, the specified IP address
    /// portion must be properly truncated (i.e. all the host bits must
    /// be zero) or the input is considered malformed. For example,
    /// "192.0.2.0/24" is accepted but "192.0.2.1/24" is not. Similarly,
    /// for IPv6, "2001:db8::/32" is accepted whereas "2001:db8::1/32"
    /// is not. The originating IP of a request must be in one of the
    /// listed subnets in order for this Condition to be true.
    /// If empty, all IP addresses are allowed.
    #[builder(into, default)]
    #[serde(rename = "ipSubnetworks")]
    pub r#ip_subnetworks: Box<Option<Vec<String>>>,
    /// An allowed list of members (users, service accounts).
    /// Using groups is not supported yet.
    /// The signed-in user originating the request must be a part of one
    /// of the provided members. If not specified, a request may come
    /// from any user (logged in/not logged in, not present in any
    /// groups, etc.).
    /// Formats: `user:{emailid}`, `serviceAccount:{emailid}`
    #[builder(into, default)]
    #[serde(rename = "members")]
    pub r#members: Box<Option<Vec<String>>>,
    /// Whether to negate the Condition. If true, the Condition becomes
    /// a NAND over its non-empty fields, each field must be false for
    /// the Condition overall to be satisfied. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "negate")]
    pub r#negate: Box<Option<bool>>,
    /// The request must originate from one of the provided
    /// countries/regions.
    /// Format: A valid ISO 3166-1 alpha-2 code.
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<String>>>,
    /// A list of other access levels defined in the same Policy,
    /// referenced by resource name. Referencing an AccessLevel which
    /// does not exist is an error. All access levels listed must be
    /// granted for the Condition to be true.
    /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
    #[builder(into, default)]
    #[serde(rename = "requiredAccessLevels")]
    pub r#required_access_levels: Box<Option<Vec<String>>>,
    /// The request must originate from one of the provided VPC networks in Google Cloud. Cannot specify this field together with `ip_subnetworks`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcNetworkSources")]
    pub r#vpc_network_sources: Box<Option<Vec<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasicConditionVpcNetworkSource>>>,
}
