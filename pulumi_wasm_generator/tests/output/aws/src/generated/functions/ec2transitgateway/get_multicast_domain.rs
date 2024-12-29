pub mod get_multicast_domain {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMulticastDomainArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetMulticastDomainFilter,
                >,
            >,
        >,
        /// Key-value tags for the EC2 Transit Gateway Multicast Domain.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway Multicast Domain.
        #[builder(into, default)]
        pub transit_gateway_multicast_domain_id: pulumi_wasm_rust::Output<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetMulticastDomainResult {
        /// EC2 Transit Gateway Multicast Domain ARN.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// EC2 Transit Gateway Multicast Domain Associations
        pub associations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ec2transitgateway::GetMulticastDomainAssociation,
            >,
        >,
        /// Whether to automatically accept cross-account subnet associations that are associated with the EC2 Transit Gateway Multicast Domain.
        pub auto_accept_shared_associations: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetMulticastDomainFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether to enable Internet Group Management Protocol (IGMP) version 2 for the EC2 Transit Gateway Multicast Domain.
        pub igmpv2_support: pulumi_wasm_rust::Output<String>,
        /// EC2 Multicast Domain Group Members
        pub members: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2transitgateway::GetMulticastDomainMember>,
        >,
        /// Identifier of the AWS account that owns the EC2 Transit Gateway Multicast Domain.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// EC2 Multicast Domain Group Sources
        pub sources: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::ec2transitgateway::GetMulticastDomainSource>,
        >,
        pub state: pulumi_wasm_rust::Output<String>,
        /// Whether to enable support for statically configuring multicast group sources for the EC2 Transit Gateway Multicast Domain.
        pub static_sources_support: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway Multicast Domain.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The ID of the transit gateway attachment.
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// EC2 Transit Gateway identifier.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        pub transit_gateway_multicast_domain_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetMulticastDomainArgs) -> GetMulticastDomainResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_multicast_domain_id_binding = args
            .transit_gateway_multicast_domain_id
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getMulticastDomain:getMulticastDomain".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayMulticastDomainId".into(),
                    value: &transit_gateway_multicast_domain_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associations".into(),
                },
                register_interface::ResultField {
                    name: "autoAcceptSharedAssociations".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "igmpv2Support".into(),
                },
                register_interface::ResultField {
                    name: "members".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "sources".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "staticSourcesSupport".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayAttachmentId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayMulticastDomainId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetMulticastDomainResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associations").unwrap(),
            ),
            auto_accept_shared_associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoAcceptSharedAssociations").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            igmpv2_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("igmpv2Support").unwrap(),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("members").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sources").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            static_sources_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticSourcesSupport").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transit_gateway_attachment_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayAttachmentId").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            transit_gateway_multicast_domain_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayMulticastDomainId").unwrap(),
            ),
        }
    }
}
