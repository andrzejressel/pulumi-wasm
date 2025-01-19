/// Represents an Address resource.
///
/// Each virtual machine instance has an ephemeral internal IP address and,
/// optionally, an external IP address. To communicate between instances on
/// the same network, you can use an instance's internal IP address. To
/// communicate with the Internet and instances outside of the same network,
/// you must specify the instance's external IP address.
///
/// Internal IP addresses are ephemeral and only belong to an instance for
/// the lifetime of the instance; if the instance is deleted and recreated,
/// the instance is assigned a new internal IP address, either by Compute
/// Engine or by you. External IP addresses can be either ephemeral or
/// static.
///
///
/// To get more information about Address, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/beta/addresses)
/// * How-to Guides
///     * [Reserving a Static External IP Address](https://cloud.google.com/compute/docs/instances-and-network)
///     * [Reserving a Static Internal IP Address](https://cloud.google.com/compute/docs/ip-addresses/reserve-static-internal-ip-address)
///
/// ## Example Usage
///
/// ### Address Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let ipAddress = address::create(
///         "ipAddress",
///         AddressArgs::builder().name("my-address").build_struct(),
///     );
/// }
/// ```
/// ### Address With Subnetwork
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = network::create(
///         "default",
///         NetworkArgs::builder().name("my-network").build_struct(),
///     );
///     let defaultSubnetwork = subnetwork::create(
///         "defaultSubnetwork",
///         SubnetworkArgs::builder()
///             .ip_cidr_range("10.0.0.0/16")
///             .name("my-subnet")
///             .network("${default.id}")
///             .region("us-central1")
///             .build_struct(),
///     );
///     let internalWithSubnetAndAddress = address::create(
///         "internalWithSubnetAndAddress",
///         AddressArgs::builder()
///             .address("10.0.42.42")
///             .address_type("INTERNAL")
///             .name("my-internal-address")
///             .region("us-central1")
///             .subnetwork("${defaultSubnetwork.id}")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Address With Gce Endpoint
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let internalWithGceEndpoint = address::create(
///         "internalWithGceEndpoint",
///         AddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("my-internal-address-")
///             .purpose("GCE_ENDPOINT")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Instance With Ip
///
///
/// ```yaml
/// resources:
///   static:
///     type: gcp:compute:Address
///     properties:
///       name: ipv4-address
///   instanceWithIp:
///     type: gcp:compute:Instance
///     name: instance_with_ip
///     properties:
///       name: vm-instance
///       machineType: f1-micro
///       zone: us-central1-a
///       bootDisk:
///         initializeParams:
///           image: ${debianImage.selfLink}
///       networkInterfaces:
///         - network: default
///           accessConfigs:
///             - natIp: ${static.address}
/// variables:
///   debianImage:
///     fn::invoke:
///       function: gcp:compute:getImage
///       arguments:
///         family: debian-11
///         project: debian-cloud
/// ```
/// ### Compute Address Ipsec Interconnect
///
///
/// ```yaml
/// resources:
///   ipsec-interconnect-address:
///     type: gcp:compute:Address
///     properties:
///       name: test-address
///       addressType: INTERNAL
///       purpose: IPSEC_INTERCONNECT
///       address: 192.168.1.0
///       prefixLength: 29
///       network: ${network.selfLink}
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: test-network
///       autoCreateSubnetworks: false
/// ```
///
/// ## Import
///
/// Address can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/addresses/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Address can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/address:Address default projects/{{project}}/regions/{{region}}/addresses/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/address:Address default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/address:Address default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/address:Address default {{name}}
/// ```
///
pub mod address {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AddressArgs {
        /// The static external IP address represented by this resource.
        /// The IP address must be inside the specified subnetwork,
        /// if any. Set by the API if undefined.
        #[builder(into, default)]
        pub address: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of address to reserve.
        /// Note: if you set this argument's value as `INTERNAL` you need to leave the `network_tier` argument unset in that resource block.
        /// Default value is `EXTERNAL`.
        /// Possible values are: `INTERNAL`, `EXTERNAL`.
        #[builder(into, default)]
        pub address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP Version that will be used by this address. The default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        #[builder(into, default)]
        pub ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The endpoint type of this address, which should be VM or NETLB. This is
        /// used for deciding which type of endpoint this address can be used after
        /// the external IPv6 address reservation.
        /// Possible values are: `VM`, `NETLB`.
        #[builder(into, default)]
        pub ipv6_endpoint_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Labels to apply to this address.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of the network in which to reserve the address. This field
        /// can only be used with INTERNAL type with the VPC_PEERING and
        /// IPSEC_INTERCONNECT purposes.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// The networking tier used for configuring this address. If this field is not
        /// specified, it is assumed to be PREMIUM.
        /// This argument should not be used when configuring Internal addresses, because [network tier cannot be set for internal traffic; it's always Premium](https://cloud.google.com/network-tiers/docs/overview).
        /// Possible values are: `PREMIUM`, `STANDARD`.
        #[builder(into, default)]
        pub network_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The prefix length if the resource represents an IP range.
        #[builder(into, default)]
        pub prefix_length: pulumi_wasm_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The purpose of this resource, which can be one of the following values.
        /// * GCE_ENDPOINT for addresses that are used by VM instances, alias IP
        /// ranges, load balancers, and similar resources.
        /// * SHARED_LOADBALANCER_VIP for an address that can be used by multiple
        /// internal load balancers.
        /// * VPC_PEERING for addresses that are reserved for VPC peer networks.
        /// * IPSEC_INTERCONNECT for addresses created from a private IP range that
        /// are reserved for a VLAN attachment in an HA VPN over Cloud Interconnect
        /// configuration. These addresses are regional resources.
        /// * PRIVATE_SERVICE_CONNECT for a private network address that is used to
        /// configure Private Service Connect. Only global internal addresses can use
        /// this purpose.
        /// This should only be set when using an Internal address.
        #[builder(into, default)]
        pub purpose: pulumi_wasm_rust::Output<Option<String>>,
        /// The Region in which the created address should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// The URL of the subnetwork in which to reserve the address. If an IP
        /// address is specified, it must be within the subnetwork's IP range.
        /// This field can only be used with INTERNAL type with
        /// GCE_ENDPOINT/DNS_RESOLVER purposes.
        #[builder(into, default)]
        pub subnetwork: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AddressResult {
        /// The static external IP address represented by this resource.
        /// The IP address must be inside the specified subnetwork,
        /// if any. Set by the API if undefined.
        pub address: pulumi_wasm_rust::Output<String>,
        /// The type of address to reserve.
        /// Note: if you set this argument's value as `INTERNAL` you need to leave the `network_tier` argument unset in that resource block.
        /// Default value is `EXTERNAL`.
        /// Possible values are: `INTERNAL`, `EXTERNAL`.
        pub address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Creation timestamp in RFC3339 text format.
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The IP Version that will be used by this address. The default value is `IPV4`.
        /// Possible values are: `IPV4`, `IPV6`.
        pub ip_version: pulumi_wasm_rust::Output<Option<String>>,
        /// The endpoint type of this address, which should be VM or NETLB. This is
        /// used for deciding which type of endpoint this address can be used after
        /// the external IPv6 address reservation.
        /// Possible values are: `VM`, `NETLB`.
        pub ipv6_endpoint_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The fingerprint used for optimistic locking of this resource.  Used
        /// internally during updates.
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Labels to apply to this address.  A list of key->value pairs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Name of the resource. The name must be 1-63 characters long, and
        /// comply with RFC1035. Specifically, the name must be 1-63 characters
        /// long and match the regular expression `a-z?`
        /// which means the first character must be a lowercase letter, and all
        /// following characters must be a dash, lowercase letter, or digit,
        /// except the last character, which cannot be a dash.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// The URL of the network in which to reserve the address. This field
        /// can only be used with INTERNAL type with the VPC_PEERING and
        /// IPSEC_INTERCONNECT purposes.
        pub network: pulumi_wasm_rust::Output<Option<String>>,
        /// The networking tier used for configuring this address. If this field is not
        /// specified, it is assumed to be PREMIUM.
        /// This argument should not be used when configuring Internal addresses, because [network tier cannot be set for internal traffic; it's always Premium](https://cloud.google.com/network-tiers/docs/overview).
        /// Possible values are: `PREMIUM`, `STANDARD`.
        pub network_tier: pulumi_wasm_rust::Output<String>,
        /// The prefix length if the resource represents an IP range.
        pub prefix_length: pulumi_wasm_rust::Output<i32>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The purpose of this resource, which can be one of the following values.
        /// * GCE_ENDPOINT for addresses that are used by VM instances, alias IP
        /// ranges, load balancers, and similar resources.
        /// * SHARED_LOADBALANCER_VIP for an address that can be used by multiple
        /// internal load balancers.
        /// * VPC_PEERING for addresses that are reserved for VPC peer networks.
        /// * IPSEC_INTERCONNECT for addresses created from a private IP range that
        /// are reserved for a VLAN attachment in an HA VPN over Cloud Interconnect
        /// configuration. These addresses are regional resources.
        /// * PRIVATE_SERVICE_CONNECT for a private network address that is used to
        /// configure Private Service Connect. Only global internal addresses can use
        /// this purpose.
        /// This should only be set when using an Internal address.
        pub purpose: pulumi_wasm_rust::Output<String>,
        /// The Region in which the created address should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        /// The URL of the subnetwork in which to reserve the address. If an IP
        /// address is specified, it must be within the subnetwork's IP range.
        /// This field can only be used with INTERNAL type with
        /// GCE_ENDPOINT/DNS_RESOLVER purposes.
        pub subnetwork: pulumi_wasm_rust::Output<String>,
        /// The URLs of the resources that are using this address.
        pub users: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: AddressArgs) -> AddressResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_binding = args.address.get_inner();
        let address_type_binding = args.address_type.get_inner();
        let description_binding = args.description.get_inner();
        let ip_version_binding = args.ip_version.get_inner();
        let ipv6_endpoint_type_binding = args.ipv6_endpoint_type.get_inner();
        let labels_binding = args.labels.get_inner();
        let name_binding = args.name.get_inner();
        let network_binding = args.network.get_inner();
        let network_tier_binding = args.network_tier.get_inner();
        let prefix_length_binding = args.prefix_length.get_inner();
        let project_binding = args.project.get_inner();
        let purpose_binding = args.purpose.get_inner();
        let region_binding = args.region.get_inner();
        let subnetwork_binding = args.subnetwork.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:compute/address:Address".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "address".into(),
                    value: &address_binding,
                },
                register_interface::ObjectField {
                    name: "addressType".into(),
                    value: &address_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipVersion".into(),
                    value: &ip_version_binding,
                },
                register_interface::ObjectField {
                    name: "ipv6EndpointType".into(),
                    value: &ipv6_endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "networkTier".into(),
                    value: &network_tier_binding,
                },
                register_interface::ObjectField {
                    name: "prefixLength".into(),
                    value: &prefix_length_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "purpose".into(),
                    value: &purpose_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "subnetwork".into(),
                    value: &subnetwork_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "address".into(),
                },
                register_interface::ResultField {
                    name: "addressType".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "ipVersion".into(),
                },
                register_interface::ResultField {
                    name: "ipv6EndpointType".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkTier".into(),
                },
                register_interface::ResultField {
                    name: "prefixLength".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "purpose".into(),
                },
                register_interface::ResultField {
                    name: "region".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "subnetwork".into(),
                },
                register_interface::ResultField {
                    name: "users".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AddressResult {
            address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("address").unwrap(),
            ),
            address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressType").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            ip_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipVersion").unwrap(),
            ),
            ipv6_endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6EndpointType").unwrap(),
            ),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkTier").unwrap(),
            ),
            prefix_length: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("prefixLength").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            purpose: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("purpose").unwrap(),
            ),
            region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("region").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            subnetwork: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetwork").unwrap(),
            ),
            users: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("users").unwrap(),
            ),
        }
    }
}
