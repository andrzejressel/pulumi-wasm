/// Creates a Global Accelerator custom routing accelerator.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod custom_routing_accelerator {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomRoutingAcceleratorArgs {
        /// The attributes of the accelerator. Fields documented below.
        #[builder(into, default)]
        pub attributes: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::globalaccelerator::CustomRoutingAcceleratorAttributes,
            >,
        >,
        /// Indicates whether the accelerator is enabled. Defaults to `true`. Valid values: `true`, `false`.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The IP address type that an accelerator supports. For a custom routing accelerator, the value must be `"IPV4"`.
        #[builder(into, default)]
        pub ip_address_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IP addresses to use for BYOIP accelerators. If not specified, the service assigns IP addresses. Valid values: 1 or 2 IPv4 addresses.
        #[builder(into, default)]
        pub ip_addresses: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The name of a custom routing accelerator.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CustomRoutingAcceleratorResult {
        /// The attributes of the accelerator. Fields documented below.
        pub attributes: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::globalaccelerator::CustomRoutingAcceleratorAttributes,
            >,
        >,
        /// The DNS name of the accelerator. For example, `a5d53ff5ee6bca4ce.awsglobalaccelerator.com`.
        pub dns_name: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether the accelerator is enabled. Defaults to `true`. Valid values: `true`, `false`.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// -  The Global Accelerator Route 53 zone ID that can be used to
        /// route an [Alias Resource Record Set](https://docs.aws.amazon.com/Route53/latest/APIReference/API_AliasTarget.html) to the Global Accelerator. This attribute
        /// is simply an alias for the zone ID `Z2BJ6XQ5FK7U4H`.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// The IP address type that an accelerator supports. For a custom routing accelerator, the value must be `"IPV4"`.
        pub ip_address_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IP addresses to use for BYOIP accelerators. If not specified, the service assigns IP addresses. Valid values: 1 or 2 IPv4 addresses.
        pub ip_addresses: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// IP address set associated with the accelerator.
        pub ip_sets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::globalaccelerator::CustomRoutingAcceleratorIpSet>,
        >,
        /// The name of a custom routing accelerator.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomRoutingAcceleratorArgs,
    ) -> CustomRoutingAcceleratorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attributes_binding = args.attributes.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let ip_address_type_binding = args.ip_address_type.get_output(context);
        let ip_addresses_binding = args.ip_addresses.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:globalaccelerator/customRoutingAccelerator:CustomRoutingAccelerator"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attributes".into(),
                    value: &attributes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddressType".into(),
                    value: &ip_address_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddresses".into(),
                    value: &ip_addresses_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomRoutingAcceleratorResult {
            attributes: o.get_field("attributes"),
            dns_name: o.get_field("dnsName"),
            enabled: o.get_field("enabled"),
            hosted_zone_id: o.get_field("hostedZoneId"),
            ip_address_type: o.get_field("ipAddressType"),
            ip_addresses: o.get_field("ipAddresses"),
            ip_sets: o.get_field("ipSets"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
