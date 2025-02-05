pub mod get_connection {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// ID of the specific connection to retrieve.
        #[builder(into)]
        pub connection_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the Global Network of the connection to retrieve.
        #[builder(into)]
        pub global_network_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value tags for the connection.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        /// ARN of the connection.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// ID of the second device in the connection.
        pub connected_device_id: pulumi_wasm_rust::Output<String>,
        /// ID of the link for the second device.
        pub connected_link_id: pulumi_wasm_rust::Output<String>,
        pub connection_id: pulumi_wasm_rust::Output<String>,
        /// Description of the connection.
        pub description: pulumi_wasm_rust::Output<String>,
        /// ID of the first device in the connection.
        pub device_id: pulumi_wasm_rust::Output<String>,
        pub global_network_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// ID of the link for the first device.
        pub link_id: pulumi_wasm_rust::Output<String>,
        /// Key-value tags for the connection.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConnectionArgs,
    ) -> GetConnectionResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_id_binding = args.connection_id.get_output(context).get_inner();
        let global_network_id_binding = args
            .global_network_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:networkmanager/getConnection:getConnection".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionId".into(),
                    value: &connection_id_binding,
                },
                register_interface::ObjectField {
                    name: "globalNetworkId".into(),
                    value: &global_network_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConnectionResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            connected_device_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectedDeviceId"),
            ),
            connected_link_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectedLinkId"),
            ),
            connection_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            device_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceId"),
            ),
            global_network_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("globalNetworkId"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            link_id: pulumi_wasm_rust::__private::into_domain(o.extract_field("linkId")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
