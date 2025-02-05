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
pub mod customer_gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CustomerGatewayArgs {
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `1` to `2147483647`. Conflicts with `bgp_asn_extended`.
        #[builder(into, default)]
        pub bgp_asn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `2147483648` to `4294967295` Conflicts with `bgp_asn`.
        #[builder(into, default)]
        pub bgp_asn_extended: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Amazon Resource Name (ARN) for the customer gateway certificate.
        #[builder(into, default)]
        pub certificate_arn: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A name for the customer gateway device.
        #[builder(into, default)]
        pub device_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address for the customer gateway device's outside interface.
        #[builder(into, default)]
        pub ip_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Tags to apply to the gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of customer gateway. The only type AWS
        /// supports at this time is "ipsec.1".
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CustomerGatewayResult {
        /// The ARN of the customer gateway.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `1` to `2147483647`. Conflicts with `bgp_asn_extended`.
        pub bgp_asn: pulumi_wasm_rust::Output<Option<String>>,
        /// The gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN). Valid values are from  `2147483648` to `4294967295` Conflicts with `bgp_asn`.
        pub bgp_asn_extended: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) for the customer gateway certificate.
        pub certificate_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// A name for the customer gateway device.
        pub device_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The IPv4 address for the customer gateway device's outside interface.
        pub ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// Tags to apply to the gateway. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of customer gateway. The only type AWS
        /// supports at this time is "ipsec.1".
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: CustomerGatewayArgs,
    ) -> CustomerGatewayResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bgp_asn_binding = args.bgp_asn.get_output(context).get_inner();
        let bgp_asn_extended_binding = args
            .bgp_asn_extended
            .get_output(context)
            .get_inner();
        let certificate_arn_binding = args
            .certificate_arn
            .get_output(context)
            .get_inner();
        let device_name_binding = args.device_name.get_output(context).get_inner();
        let ip_address_binding = args.ip_address.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/customerGateway:CustomerGateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bgpAsn".into(),
                    value: &bgp_asn_binding,
                },
                register_interface::ObjectField {
                    name: "bgpAsnExtended".into(),
                    value: &bgp_asn_extended_binding,
                },
                register_interface::ObjectField {
                    name: "certificateArn".into(),
                    value: &certificate_arn_binding,
                },
                register_interface::ObjectField {
                    name: "deviceName".into(),
                    value: &device_name_binding,
                },
                register_interface::ObjectField {
                    name: "ipAddress".into(),
                    value: &ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CustomerGatewayResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            bgp_asn: pulumi_wasm_rust::__private::into_domain(o.extract_field("bgpAsn")),
            bgp_asn_extended: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bgpAsnExtended"),
            ),
            certificate_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            device_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceName"),
            ),
            ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
