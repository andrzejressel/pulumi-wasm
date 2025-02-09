/// Provides a customer gateway inside a VPC. These objects can be connected to VPN gateways via VPN connections, and allow you to establish tunnels between your network and the VPC.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   main:
///     type: aws:ec2:CustomerGateway
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.83.124.10
///       type: ipsec.1
///       tags:
///         Name: main-customer-gateway
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Customer Gateways using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/customerGateway:CustomerGateway main cgw-b4dc3961
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod customer_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomerGatewayArgs {
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `1` to `2147483647`. Conflicts with `bgp_asn_extended`.
        #[builder(into, default)]
        pub bgp_asn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `2147483648` to `4294967295` Conflicts with `bgp_asn`.
        #[builder(into, default)]
        pub bgp_asn_extended: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the customer gateway certificate.
        #[builder(into, default)]
        pub certificate_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A name for the customer gateway device.
        #[builder(into, default)]
        pub device_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address for the customer gateway device's outside interface.
        #[builder(into, default)]
        pub ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of customer gateway. The only type AWS
        /// supports at this time is "ipsec.1".
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomerGatewayResult {
        /// The ARN of the customer gateway.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `1` to `2147483647`. Conflicts with `bgp_asn_extended`.
        pub bgp_asn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `2147483648` to `4294967295` Conflicts with `bgp_asn`.
        pub bgp_asn_extended: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the customer gateway certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// A name for the customer gateway device.
        pub device_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 address for the customer gateway device's outside interface.
        pub ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// Tags to apply to the gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of customer gateway. The only type AWS
        /// supports at this time is "ipsec.1".
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CustomerGatewayArgs,
    ) -> CustomerGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bgp_asn_binding = args.bgp_asn.get_output(context);
        let bgp_asn_extended_binding = args.bgp_asn_extended.get_output(context);
        let certificate_arn_binding = args.certificate_arn.get_output(context);
        let device_name_binding = args.device_name.get_output(context);
        let ip_address_binding = args.ip_address.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/customerGateway:CustomerGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpAsn".into(),
                    value: bgp_asn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpAsnExtended".into(),
                    value: bgp_asn_extended_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateArn".into(),
                    value: certificate_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceName".into(),
                    value: device_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipAddress".into(),
                    value: ip_address_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CustomerGatewayResult {
            arn: o.get_field("arn"),
            bgp_asn: o.get_field("bgpAsn"),
            bgp_asn_extended: o.get_field("bgpAsnExtended"),
            certificate_arn: o.get_field("certificateArn"),
            device_name: o.get_field("deviceName"),
            ip_address: o.get_field("ipAddress"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}
