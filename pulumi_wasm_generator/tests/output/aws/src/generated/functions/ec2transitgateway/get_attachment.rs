pub mod get_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetAttachmentArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetAttachmentFilter>,
            >,
        >,
        /// Key-value tags for the attachment.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// ID of the attachment.
        #[builder(into, default)]
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetAttachmentResult {
        /// ARN of the attachment.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The state of the association (see [the underlying AWS API](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_TransitGatewayAttachmentAssociation.html) for valid values).
        pub association_state: pulumi_wasm_rust::Output<String>,
        /// The ID of the route table for the transit gateway.
        pub association_transit_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::super::types::ec2transitgateway::GetAttachmentFilter>,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ID of the resource.
        pub resource_id: pulumi_wasm_rust::Output<String>,
        /// ID of the AWS account that owns the resource.
        pub resource_owner_id: pulumi_wasm_rust::Output<String>,
        /// Resource type.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// Attachment state.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the attachment.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub transit_gateway_attachment_id: pulumi_wasm_rust::Output<String>,
        /// ID of the transit gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the AWS account that owns the transit gateway.
        pub transit_gateway_owner_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetAttachmentArgs) -> GetAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let tags_binding = args.tags.get_inner();
        let transit_gateway_attachment_id_binding = args
            .transit_gateway_attachment_id
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getAttachment:getAttachment".into(),
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
                    name: "transitGatewayAttachmentId".into(),
                    value: &transit_gateway_attachment_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "associationState".into(),
                },
                register_interface::ResultField {
                    name: "associationTransitGatewayRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "resourceId".into(),
                },
                register_interface::ResultField {
                    name: "resourceOwnerId".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
                    name: "transitGatewayOwnerId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetAttachmentResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            association_state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationState").unwrap(),
            ),
            association_transit_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("associationTransitGatewayRouteTableId").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceId").unwrap(),
            ),
            resource_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceOwnerId").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
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
            transit_gateway_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayOwnerId").unwrap(),
            ),
        }
    }
}
