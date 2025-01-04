/// Creates a Global Accelerator custom routing accelerator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = custom_routing_accelerator::create(
///         "example",
///         CustomRoutingAcceleratorArgs::builder()
///             .attributes(
///                 CustomRoutingAcceleratorAttributes::builder()
///                     .flowLogsEnabled(true)
///                     .flowLogsS3Bucket("example-bucket")
///                     .flowLogsS3Prefix("flow-logs/")
///                     .build_struct(),
///             )
///             .enabled(true)
///             .ip_address_type("IPV4")
///             .ip_addresses(vec!["1.2.3.4",])
///             .name("Example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Global Accelerator custom routing accelerators using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/customRoutingAccelerator:CustomRoutingAccelerator example arn:aws:globalaccelerator::111111111111:accelerator/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
pub mod custom_routing_accelerator {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomRoutingAcceleratorArgs {
        /// The attributes of the accelerator. Fields documented below.
        #[builder(into, default)]
        pub attributes: pulumi_wasm_rust::Output<
            Option<
                super::super::types::globalaccelerator::CustomRoutingAcceleratorAttributes,
            >,
        >,
        /// Indicates whether the accelerator is enabled. Defaults to `true`. Valid values: `true`, `false`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The IP address type that an accelerator supports. For a custom routing accelerator, the value must be `"IPV4"`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP addresses to use for BYOIP accelerators. If not specified, the service assigns IP addresses. Valid values: 1 or 2 IPv4 addresses.
        #[builder(into, default)]
        pub ip_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The name of a custom routing accelerator.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomRoutingAcceleratorResult {
        /// The attributes of the accelerator. Fields documented below.
        pub attributes: pulumi_wasm_rust::Output<
            Option<
                super::super::types::globalaccelerator::CustomRoutingAcceleratorAttributes,
            >,
        >,
        /// The DNS name of the accelerator. For example, `a5d53ff5ee6bca4ce.awsglobalaccelerator.com`.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// Indicates whether the accelerator is enabled. Defaults to `true`. Valid values: `true`, `false`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// -  The Global Accelerator Route 53 zone ID that can be used to
        /// route an [Alias Resource Record Set](https://docs.aws.amazon.com/Route53/latest/APIReference/API_AliasTarget.html) to the Global Accelerator. This attribute
        /// is simply an alias for the zone ID `Z2BJ6XQ5FK7U4H`.
        pub hosted_zone_id: pulumi_wasm_rust::Output<String>,
        /// The IP address type that an accelerator supports. For a custom routing accelerator, the value must be `"IPV4"`.
        pub ip_address_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The IP addresses to use for BYOIP accelerators. If not specified, the service assigns IP addresses. Valid values: 1 or 2 IPv4 addresses.
        pub ip_addresses: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// IP address set associated with the accelerator.
        pub ip_sets: pulumi_wasm_rust::Output<
            Vec<super::super::types::globalaccelerator::CustomRoutingAcceleratorIpSet>,
        >,
        /// The name of a custom routing accelerator.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: CustomRoutingAcceleratorArgs,
    ) -> CustomRoutingAcceleratorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let attributes_binding = args.attributes.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let ip_address_type_binding = args.ip_address_type.get_inner();
        let ip_addresses_binding = args.ip_addresses.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:globalaccelerator/customRoutingAccelerator:CustomRoutingAccelerator"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddresses".into(),
                    value: &ip_addresses_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "attributes".into(),
                },
                register_interface::ResultField {
                    name: "dnsName".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "hostedZoneId".into(),
                },
                register_interface::ResultField {
                    name: "ipAddressType".into(),
                },
                register_interface::ResultField {
                    name: "ipAddresses".into(),
                },
                register_interface::ResultField {
                    name: "ipSets".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CustomRoutingAcceleratorResult {
            attributes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("attributes").unwrap(),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsName").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            hosted_zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostedZoneId").unwrap(),
            ),
            ip_address_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddressType").unwrap(),
            ),
            ip_addresses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipAddresses").unwrap(),
            ),
            ip_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipSets").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
