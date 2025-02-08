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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRoutingProfileArgs,
    ) -> GetRoutingProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let routing_profile_id_binding = args
            .routing_profile_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getRoutingProfile:getRoutingProfile".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "routingProfileId".into(),
                    value: &routing_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRoutingProfileResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            default_outbound_queue_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultOutboundQueueId"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            media_concurrencies: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("mediaConcurrencies"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            queue_configs: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("queueConfigs"),
            ),
            routing_profile_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routingProfileId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
