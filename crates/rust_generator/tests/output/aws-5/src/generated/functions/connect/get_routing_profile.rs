#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_routing_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoutingProfileArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Returns information on a specific Routing Profile by name
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Returns information on a specific Routing Profile by Routing Profile id
        #[builder(into, default)]
        pub routing_profile_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Routing Profile.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRoutingProfileResult {
        /// ARN of the Routing Profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies the default outbound queue for the Routing Profile.
        pub default_outbound_queue_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the Routing Profile.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        /// One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
        pub media_concurrencies: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetRoutingProfileMediaConcurrency>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
        pub queue_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::connect::GetRoutingProfileQueueConfig>,
        >,
        pub routing_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to the Routing Profile.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetRoutingProfileArgs,
    ) -> GetRoutingProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let instance_id_binding = args.instance_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let routing_profile_id_binding = args.routing_profile_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:connect/getRoutingProfile:getRoutingProfile".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceId".into(),
                    value: instance_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routingProfileId".into(),
                    value: routing_profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetRoutingProfileResult {
            arn: o.get_field("arn"),
            default_outbound_queue_id: o.get_field("defaultOutboundQueueId"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            instance_id: o.get_field("instanceId"),
            media_concurrencies: o.get_field("mediaConcurrencies"),
            name: o.get_field("name"),
            queue_configs: o.get_field("queueConfigs"),
            routing_profile_id: o.get_field("routingProfileId"),
            tags: o.get_field("tags"),
        }
    }
}
