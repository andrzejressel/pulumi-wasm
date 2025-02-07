pub mod get_customer_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCustomerGatewayArgs {
        /// One or more [name-value pairs][dcg-filters] to filter by.
        ///
        /// [dcg-filters]: https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeCustomerGateways.html
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetCustomerGatewayFilter>>,
        >,
        /// ID of the gateway.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of key-value pairs assigned to the gateway.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCustomerGatewayResult {
        /// ARN of the customer gateway.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN).
        pub bgp_asn: pulumi_gestalt_rust::Output<i32>,
        /// Gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN).
        pub bgp_asn_extended: pulumi_gestalt_rust::Output<i32>,
        /// ARN for the customer gateway certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// Name for the customer gateway device.
        pub device_name: pulumi_gestalt_rust::Output<String>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetCustomerGatewayFilter>>,
        >,
        pub id: pulumi_gestalt_rust::Output<String>,
        /// IP address of the gateway's Internet-routable external interface.
        pub ip_address: pulumi_gestalt_rust::Output<String>,
        /// Map of key-value pairs assigned to the gateway.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of customer gateway. The only type AWS supports at this time is "ipsec.1".
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCustomerGatewayArgs,
    ) -> GetCustomerGatewayResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getCustomerGateway:getCustomerGateway".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCustomerGatewayResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            bgp_asn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpAsn"),
            ),
            bgp_asn_extended: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("bgpAsnExtended"),
            ),
            certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            device_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deviceName"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            ip_address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipAddress"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_gestalt_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
