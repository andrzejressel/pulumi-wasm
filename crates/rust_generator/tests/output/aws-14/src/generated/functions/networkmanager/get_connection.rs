#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_connection {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConnectionArgs {
        /// ID of the specific connection to retrieve.
        #[builder(into)]
        pub connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Global Network of the connection to retrieve.
        #[builder(into)]
        pub global_network_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value tags for the connection.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConnectionResult {
        /// ARN of the connection.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// ID of the second device in the connection.
        pub connected_device_id: pulumi_gestalt_rust::Output<String>,
        /// ID of the link for the second device.
        pub connected_link_id: pulumi_gestalt_rust::Output<String>,
        pub connection_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the connection.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// ID of the first device in the connection.
        pub device_id: pulumi_gestalt_rust::Output<String>,
        pub global_network_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// ID of the link for the first device.
        pub link_id: pulumi_gestalt_rust::Output<String>,
        /// Key-value tags for the connection.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConnectionArgs,
    ) -> GetConnectionResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_id_binding = args.connection_id.get_output(context);
        let global_network_id_binding = args.global_network_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:networkmanager/getConnection:getConnection".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionId".into(),
                    value: connection_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "globalNetworkId".into(),
                    value: global_network_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConnectionResult {
            arn: o.get_field("arn"),
            connected_device_id: o.get_field("connectedDeviceId"),
            connected_link_id: o.get_field("connectedLinkId"),
            connection_id: o.get_field("connectionId"),
            description: o.get_field("description"),
            device_id: o.get_field("deviceId"),
            global_network_id: o.get_field("globalNetworkId"),
            id: o.get_field("id"),
            link_id: o.get_field("linkId"),
            tags: o.get_field("tags"),
        }
    }
}
