pub mod get_peering_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPeeringAttachmentArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetPeeringAttachmentFilter,
                >,
            >,
        >,
        /// Identifier of the EC2 Transit Gateway Peering Attachment.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
        /// Mapping of tags, each pair of which must exactly match
        /// a pair on the specific EC2 Transit Gateway Peering Attachment to retrieve.
        ///
        /// More complex filters can be expressed using one or more `filter` sub-blocks,
        /// which take the following arguments:
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPeeringAttachmentResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetPeeringAttachmentFilter,
                >,
            >,
        >,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the peer AWS account
        pub peer_account_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of the peer AWS region
        pub peer_region: pulumi_wasm_rust::Output<String>,
        /// Identifier of the peer EC2 Transit Gateway
        pub peer_transit_gateway_id: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Identifier of the local EC2 Transit Gateway
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPeeringAttachmentArgs) -> GetPeeringAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let id_binding = args.id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getPeeringAttachment:getPeeringAttachment"
                .into(),
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
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "peerAccountId".into(),
                },
                register_interface::ResultField {
                    name: "peerRegion".into(),
                },
                register_interface::ResultField {
                    name: "peerTransitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPeeringAttachmentResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            peer_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerAccountId").unwrap(),
            ),
            peer_region: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerRegion").unwrap(),
            ),
            peer_transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("peerTransitGatewayId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
        }
    }
}
