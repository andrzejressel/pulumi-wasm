#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetForwardingRulesRule {
    /// The 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.
    /// Only packets addressed to ports in the specified range will be forwarded
    /// to the backends configured with this forwarding rule.
    /// 
    /// The 'allPorts' field has the following limitations:
    /// * It requires that the forwarding rule 'IPProtocol' be TCP, UDP, SCTP, or
    /// L3_DEFAULT.
    /// * It's applicable only to the following products: internal passthrough
    /// Network Load Balancers, backend service-based external passthrough Network
    /// Load Balancers, and internal and external protocol forwarding.
    /// * Set this field to true to allow packets addressed to any port or packets
    /// lacking destination port information (for example, UDP fragments after the
    /// first fragment) to be forwarded to the backends configured with this
    /// forwarding rule. The L3_DEFAULT protocol requires 'allPorts' be set to
    /// true.
    #[builder(into)]
    #[serde(rename = "allPorts")]
    pub r#all_ports: Box<bool>,
    /// This field is used along with the 'backend_service' field for
    /// internal load balancing or with the 'target' field for internal
    /// TargetInstance.
    /// 
    /// If the field is set to 'TRUE', clients can access ILB from all
    /// regions.
    /// 
    /// Otherwise only allows access from clients in the same region as the
    /// internal load balancer.
    #[builder(into)]
    #[serde(rename = "allowGlobalAccess")]
    pub r#allow_global_access: Box<bool>,
    /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
    #[builder(into)]
    #[serde(rename = "allowPscGlobalAccess")]
    pub r#allow_psc_global_access: Box<bool>,
    /// Identifies the backend service to which the forwarding rule sends traffic.
    /// 
    /// Required for Internal TCP/UDP Load Balancing and Network Load Balancing;
    /// must be omitted for all other load balancer types.
    #[builder(into)]
    #[serde(rename = "backendService")]
    pub r#backend_service: Box<String>,
    /// [Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified.
    #[builder(into)]
    #[serde(rename = "baseForwardingRule")]
    pub r#base_forwarding_rule: Box<String>,
    /// Creation timestamp in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "creationTimestamp")]
    pub r#creation_timestamp: Box<String>,
    /// An optional description of this resource. Provide this property when
    /// you create the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Box<std::collections::HashMap<String, String>>,
    /// The unique identifier number for the resource. This identifier is defined by the server.
    #[builder(into)]
    #[serde(rename = "forwardingRuleId")]
    pub r#forwarding_rule_id: Box<i32>,
    /// IP address for which this forwarding rule accepts traffic. When a client
    /// sends traffic to this IP address, the forwarding rule directs the traffic
    /// to the referenced 'target' or 'backendService'.
    /// 
    /// While creating a forwarding rule, specifying an 'IPAddress' is
    /// required under the following circumstances:
    /// 
    /// * When the 'target' is set to 'targetGrpcProxy' and
    /// 'validateForProxyless' is set to 'true', the
    /// 'IPAddress' should be set to '0.0.0.0'.
    /// * When the 'target' is a Private Service Connect Google APIs
    /// bundle, you must specify an 'IPAddress'.
    /// 
    /// Otherwise, you can optionally specify an IP address that references an
    /// existing static (reserved) IP address resource. When omitted, Google Cloud
    /// assigns an ephemeral IP address.
    /// 
    /// Use one of the following formats to specify an IP address while creating a
    /// forwarding rule:
    /// 
    /// * IP address number, as in '100.1.2.3'
    /// * IPv6 address range, as in '2600:1234::/96'
    /// * Full resource URL, as in
    /// 'https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name'
    /// * Partial URL or by name, as in:
    ///   * 'projects/project_id/regions/region/addresses/address-name'
    ///   * 'regions/region/addresses/address-name'
    ///   * 'global/addresses/address-name'
    ///   * 'address-name'
    /// 
    /// The forwarding rule's 'target' or 'backendService',
    /// and in most cases, also the 'loadBalancingScheme', determine the
    /// type of IP address that you can use. For detailed information, see
    /// [IP address
    /// specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
    /// 
    /// When reading an 'IPAddress', the API always returns the IP
    /// address number.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// The IP protocol to which this rule applies.
    /// 
    /// For protocol forwarding, valid
    /// options are 'TCP', 'UDP', 'ESP',
    /// 'AH', 'SCTP', 'ICMP' and
    /// 'L3_DEFAULT'.
    /// 
    /// The valid IP protocols are different for different load balancing products
    /// as described in [Load balancing
    /// features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
    /// 
    /// A Forwarding Rule with protocol L3_DEFAULT can attach with target instance or
    /// backend service with UNSPECIFIED protocol.
    /// A forwarding rule with "L3_DEFAULT" IPProtocal cannot be attached to a backend service with TCP or UDP. Possible values: ["TCP", "UDP", "ESP", "AH", "SCTP", "ICMP", "L3_DEFAULT"]
    #[builder(into)]
    #[serde(rename = "ipProtocol")]
    pub r#ip_protocol: Box<String>,
    /// The IP address version that will be used by this forwarding rule.
    /// Valid options are IPV4 and IPV6.
    /// 
    /// If not set, the IPv4 address will be used by default. Possible values: ["IPV4", "IPV6"]
    #[builder(into)]
    #[serde(rename = "ipVersion")]
    pub r#ip_version: Box<String>,
    /// Indicates whether or not this load balancer can be used as a collector for
    /// packet mirroring. To prevent mirroring loops, instances behind this
    /// load balancer will not have their traffic mirrored even if a
    /// 'PacketMirroring' rule applies to them.
    /// 
    /// This can only be set to true for load balancers that have their
    /// 'loadBalancingScheme' set to 'INTERNAL'.
    #[builder(into)]
    #[serde(rename = "isMirroringCollector")]
    pub r#is_mirroring_collector: Box<bool>,
    /// The fingerprint used for optimistic locking of this resource.  Used
    /// internally during updates.
    #[builder(into)]
    #[serde(rename = "labelFingerprint")]
    pub r#label_fingerprint: Box<String>,
    /// Labels to apply to this forwarding rule.  A list of key->value pairs.
    /// 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Specifies the forwarding rule type.
    /// 
    /// For more information about forwarding rules, refer to
    /// [Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts). Default value: "EXTERNAL" Possible values: ["EXTERNAL", "EXTERNAL_MANAGED", "INTERNAL", "INTERNAL_MANAGED"]
    #[builder(into)]
    #[serde(rename = "loadBalancingScheme")]
    pub r#load_balancing_scheme: Box<String>,
    /// Name of the resource; provided by the client when the resource is created.
    /// The name must be 1-63 characters long, and comply with
    /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
    /// 
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression 'a-z?' which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    /// 
    /// For Private Service Connect forwarding rules that forward traffic to Google
    /// APIs, the forwarding rule name must be a 1-20 characters string with
    /// lowercase letters and numbers and must start with a letter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// This field is not used for external load balancing.
    /// 
    /// For Internal TCP/UDP Load Balancing, this field identifies the network that
    /// the load balanced IP should belong to for this Forwarding Rule.
    /// If the subnetwork is specified, the network of the subnetwork will be used.
    /// If neither subnetwork nor this field is specified, the default network will
    /// be used.
    /// 
    /// For Private Service Connect forwarding rules that forward traffic to Google
    /// APIs, a network must be provided.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// This signifies the networking tier used for configuring
    /// this load balancer and can only take the following values:
    /// 'PREMIUM', 'STANDARD'.
    /// 
    /// For regional ForwardingRule, the valid values are 'PREMIUM' and
    /// 'STANDARD'. For GlobalForwardingRule, the valid value is
    /// 'PREMIUM'.
    /// 
    /// If this field is not specified, it is assumed to be 'PREMIUM'.
    /// If 'IPAddress' is specified, this value must be equal to the
    /// networkTier of the Address. Possible values: ["PREMIUM", "STANDARD"]
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: Box<String>,
    /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field.
    #[builder(into)]
    #[serde(rename = "noAutomateDnsZone")]
    pub r#no_automate_dns_zone: Box<bool>,
    /// The 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.
    /// Only packets addressed to ports in the specified range will be forwarded
    /// to the backends configured with this forwarding rule.
    /// 
    /// The 'portRange' field has the following limitations:
    /// * It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,
    /// and
    /// * It's applicable only to the following products: external passthrough
    /// Network Load Balancers, internal and external proxy Network Load
    /// Balancers, internal and external Application Load Balancers, external
    /// protocol forwarding, and Classic VPN.
    /// * Some products have restrictions on what ports can be used. See
    /// [port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)
    /// for details.
    /// 
    /// For external forwarding rules, two or more forwarding rules cannot use the
    /// same '[IPAddress, IPProtocol]' pair, and cannot have overlapping
    /// 'portRange's.
    /// 
    /// For internal forwarding rules within the same VPC network, two or more
    /// forwarding rules cannot use the same '[IPAddress, IPProtocol]' pair, and
    /// cannot have overlapping 'portRange's.
    /// 
    /// @pattern: \d+(?:-\d+)?
    #[builder(into)]
    #[serde(rename = "portRange")]
    pub r#port_range: Box<String>,
    /// The 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.
    /// Only packets addressed to ports in the specified range will be forwarded
    /// to the backends configured with this forwarding rule.
    /// 
    /// The 'ports' field has the following limitations:
    /// * It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,
    /// and
    /// * It's applicable only to the following products: internal passthrough
    /// Network Load Balancers, backend service-based external passthrough Network
    /// Load Balancers, and internal protocol forwarding.
    /// * You can specify a list of up to five ports by number, separated by
    /// commas. The ports can be contiguous or discontiguous.
    /// 
    /// For external forwarding rules, two or more forwarding rules cannot use the
    /// same '[IPAddress, IPProtocol]' pair if they share at least one port
    /// number.
    /// 
    /// For internal forwarding rules within the same VPC network, two or more
    /// forwarding rules cannot use the same '[IPAddress, IPProtocol]' pair if
    /// they share at least one port number.
    /// 
    /// @pattern: \d+(?:-\d+)?
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Vec<String>>,
    /// The name of the project.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// The PSC connection id of the PSC Forwarding Rule.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: Box<String>,
    /// The PSC connection status of the PSC Forwarding Rule. Possible values: 'STATUS_UNSPECIFIED', 'PENDING', 'ACCEPTED', 'REJECTED', 'CLOSED'
    #[builder(into)]
    #[serde(rename = "pscConnectionStatus")]
    pub r#psc_connection_status: Box<String>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "recreateClosedPsc")]
    pub r#recreate_closed_psc: Box<bool>,
    /// The region you want to get the forwarding rules from.
    /// 
    /// These arguments must be set in either the provider or the resource in order for the information to be queried.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
    /// The URI of the resource.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
    /// Service Directory resources to register this forwarding rule with.
    /// 
    /// Currently, only supports a single Service Directory resource.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryRegistrations")]
    pub r#service_directory_registrations: Box<Vec<super::super::types::compute::GetForwardingRulesRuleServiceDirectoryRegistration>>,
    /// An optional prefix to the service name for this Forwarding Rule.
    /// If specified, will be the first label of the fully qualified service
    /// name.
    /// 
    /// The label must be 1-63 characters long, and comply with RFC1035.
    /// Specifically, the label must be 1-63 characters long and match the
    /// regular expression 'a-z?' which means the first
    /// character must be a lowercase letter, and all following characters
    /// must be a dash, lowercase letter, or digit, except the last
    /// character, which cannot be a dash.
    /// 
    /// This field is only used for INTERNAL load balancing.
    #[builder(into)]
    #[serde(rename = "serviceLabel")]
    pub r#service_label: Box<String>,
    /// The internal fully qualified service name for this Forwarding Rule.
    /// 
    /// This field is only used for INTERNAL load balancing.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<String>,
    /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
    #[builder(into)]
    #[serde(rename = "sourceIpRanges")]
    pub r#source_ip_ranges: Box<Vec<String>>,
    /// This field identifies the subnetwork that the load balanced IP should
    /// belong to for this Forwarding Rule, used in internal load balancing and
    /// network load balancing with IPv6.
    /// 
    /// If the network specified is in auto subnet mode, this field is optional.
    /// However, a subnetwork must be specified if the network is in custom subnet
    /// mode or when creating external forwarding rule with IPv6.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<String>,
    /// The URL of the target resource to receive the matched traffic.  For
    /// regional forwarding rules, this target must be in the same region as the
    /// forwarding rule. For global forwarding rules, this target must be a global
    /// load balancing resource.
    /// 
    /// The forwarded traffic must be of a type appropriate to the target object.
    /// *  For load balancers, see the "Target" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
    /// *  For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle:
    ///   *  'vpc-sc' - [ APIs that support VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/supported-products).
    ///   *  'all-apis' - [All supported Google APIs](https://cloud.google.com/vpc/docs/private-service-connect#supported-apis).
    /// 
    /// For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
}
