pub mod get_vpc_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpcAttachmentArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpcAttachmentFilter,
                >,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway VPC Attachment.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetVpcAttachmentResult {
        /// Whether Appliance Mode support is enabled.
        pub appliance_mode_support: pulumi_wasm_rust::Output<String>,
        /// Whether DNS support is enabled.
        pub dns_support: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpcAttachmentFilter,
                >,
            >,
        >,
        /// EC2 Transit Gateway VPC Attachment identifier
        pub id: pulumi_wasm_rust::Output<String>,
        /// Whether IPv6 support is enabled.
        pub ipv6_support: pulumi_wasm_rust::Output<String>,
        /// Whether Security Group Referencing Support is enabled.
        pub security_group_referencing_support: pulumi_wasm_rust::Output<String>,
        /// Identifiers of EC2 Subnets.
        pub subnet_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Key-value tags for the EC2 Transit Gateway VPC Attachment
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// EC2 Transit Gateway identifier
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 VPC.
        pub vpc_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the AWS account that owns the EC2 VPC.
        pub vpc_owner_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetVpcAttachmentArgs) -> GetVpcAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getVpcAttachment:getVpcAttachment".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "applianceModeSupport".into(),
                },
                register_interface::ResultField {
                    name: "dnsSupport".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipv6Support".into(),
                },
                register_interface::ResultField {
                    name: "securityGroupReferencingSupport".into(),
                },
                register_interface::ResultField {
                    name: "subnetIds".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcId".into(),
                },
                register_interface::ResultField {
                    name: "vpcOwnerId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpcAttachmentResult {
            appliance_mode_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applianceModeSupport").unwrap(),
            ),
            dns_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dnsSupport").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ipv6_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipv6Support").unwrap(),
            ),
            security_group_referencing_support: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityGroupReferencingSupport").unwrap(),
            ),
            subnet_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetIds").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpc_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcId").unwrap(),
            ),
            vpc_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcOwnerId").unwrap(),
            ),
        }
    }
}
