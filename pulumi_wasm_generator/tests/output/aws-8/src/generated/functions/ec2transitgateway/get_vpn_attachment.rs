pub mod get_vpn_attachment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVpnAttachmentArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpnAttachmentFilter,
                >,
            >,
        >,
        /// Map of tags, each pair of which must exactly match a pair on the desired Transit Gateway VPN Attachment.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the EC2 Transit Gateway.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Identifier of the EC2 VPN Connection.
        #[builder(into, default)]
        pub vpn_connection_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetVpnAttachmentResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetVpnAttachmentFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the EC2 Transit Gateway VPN Attachment
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        pub vpn_connection_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetVpnAttachmentArgs,
    ) -> GetVpnAttachmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let transit_gateway_id_binding = args
            .transit_gateway_id
            .get_output(context)
            .get_inner();
        let vpn_connection_id_binding = args
            .vpn_connection_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getVpnAttachment:getVpnAttachment".into(),
            version: super::super::super::get_version(),
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
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnConnectionId".into(),
                    value: &vpn_connection_id_binding,
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
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpnConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetVpnAttachmentResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpn_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnConnectionId").unwrap(),
            ),
        }
    }
}
