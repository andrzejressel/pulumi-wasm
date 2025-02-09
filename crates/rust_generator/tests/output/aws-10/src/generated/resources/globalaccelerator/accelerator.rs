/// Creates a Global Accelerator accelerator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = accelerator::create(
///         "example",
///         AcceleratorArgs::builder()
///             .attributes(
///                 AcceleratorAttributes::builder()
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
/// Using `pulumi import`, import Global Accelerator accelerators using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:globalaccelerator/accelerator:Accelerator example arn:aws:globalaccelerator::111111111111:accelerator/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod accelerator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AcceleratorArgs {
        /// The attributes of the accelerator. Fields documented below.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::globalaccelerator::AcceleratorAttributes>,
        >,
        /// Indicates whether the accelerator is enabled. Defaults to `true`. Valid values: `true`, `false`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The value for the address type. Defaults to `IPV4`. Valid values: `IPV4`, `DUAL_STACK`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP addresses to use for BYOIP accelerators. If not specified, the service assigns IP addresses. Valid values: 1 or 2 IPv4 addresses.
        #[builder(into, default)]
        pub ip_addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of the accelerator.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AcceleratorResult {
        /// The attributes of the accelerator. Fields documented below.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<super::super::types::globalaccelerator::AcceleratorAttributes>,
        >,
        /// The DNS name of the accelerator. For example, `a5d53ff5ee6bca4ce.awsglobalaccelerator.com`.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// The Domain Name System (DNS) name that Global Accelerator creates that points to a dual-stack accelerator's four static IP addresses: two IPv4 addresses and two IPv6 addresses. For example, `a1234567890abcdef.dualstack.awsglobalaccelerator.com`.
        pub dual_stack_dns_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the accelerator is enabled. Defaults to `true`. Valid values: `true`, `false`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// -  The Global Accelerator Route 53 zone ID that can be used to
        /// route an [Alias Resource Record Set](https://docs.aws.amazon.com/Route53/latest/APIReference/API_AliasTarget.html) to the Global Accelerator. This attribute
        /// is simply an alias for the zone ID `Z2BJ6XQ5FK7U4H`.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The value for the address type. Defaults to `IPV4`. Valid values: `IPV4`, `DUAL_STACK`.
        pub ip_address_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IP addresses to use for BYOIP accelerators. If not specified, the service assigns IP addresses. Valid values: 1 or 2 IPv4 addresses.
        pub ip_addresses: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// IP address set associated with the accelerator.
        pub ip_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::globalaccelerator::AcceleratorIpSet>,
        >,
        /// The name of the accelerator.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: AcceleratorArgs,
    ) -> AcceleratorResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let attributes_binding_1 = args.attributes.get_output(context);
        let attributes_binding = attributes_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let ip_address_type_binding_1 = args.ip_address_type.get_output(context);
        let ip_address_type_binding = ip_address_type_binding_1.get_inner();
        let ip_addresses_binding_1 = args.ip_addresses.get_output(context);
        let ip_addresses_binding = ip_addresses_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:globalaccelerator/accelerator:Accelerator".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        AcceleratorResult {
            attributes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("attributes"),
            ),
            dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            dual_stack_dns_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dualStackDnsName"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            hosted_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            ip_address_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddressType"),
            ),
            ip_addresses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddresses"),
            ),
            ip_sets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipSets"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
