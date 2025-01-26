/// Associates the specified subnet and transit gateway attachment with the specified transit gateway multicast domain.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = transit_gateway::create(
///         "example",
///         TransitGatewayArgs::builder().multicast_support("enable").build_struct(),
///     );
///     let exampleMulticastDomain = multicast_domain::create(
///         "exampleMulticastDomain",
///         MulticastDomainArgs::builder().transit_gateway_id("${example.id}").build_struct(),
///     );
///     let exampleMulticastDomainAssociation = multicast_domain_association::create(
///         "exampleMulticastDomainAssociation",
///         MulticastDomainAssociationArgs::builder()
///             .subnet_id("${exampleAwsSubnet.id}")
///             .transit_gateway_attachment_id("${exampleVpcAttachment.id}")
///             .transit_gateway_multicast_domain_id("${exampleMulticastDomain.id}")
///             .build_struct(),
///     );
///     let exampleVpcAttachment = vpc_attachment::create(
///         "exampleVpcAttachment",
///         VpcAttachmentArgs::builder()
///             .subnet_ids(vec!["${exampleAwsSubnet.id}",])
///             .transit_gateway_id("${example.id}")
///             .vpc_id("${exampleAwsVpc.id}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod multicast_domain_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MulticastDomainAssociationArgs {
        /// The ID of the subnet to associate with the transit gateway multicast domain.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the transit gateway attachment.
        #[builder(into)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the transit gateway multicast domain.
        #[builder(into)]
        pub transit_gateway_multicast_domain_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct MulticastDomainAssociationResult {
        /// The ID of the subnet to associate with the transit gateway multicast domain.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the transit gateway attachment.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the transit gateway multicast domain.
        pub transit_gateway_multicast_domain_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MulticastDomainAssociationArgs,
    ) -> MulticastDomainAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let subnet_id_binding = args.subnet_id.get_output(context).get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_output(context)
            .get_inner();
        let transit_gateway_multicast_domain_id_binding = args
            .transit_gateway_multicast_domain_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2transitgateway/multicastDomainAssociation:MulticastDomainAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayMulticastDomainId".into(),
                    value: &transit_gateway_multicast_domain_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayMulticastDomainId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MulticastDomainAssociationResult {
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_multicast_domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayMulticastDomainId").unwrap(),
            ),
        }
    }
}
