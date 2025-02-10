#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_traffic_manager_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTrafficManagerProfileArgs {
        /// Specifies the name of the Traffic Manager Profile.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the resource group the Traffic Manager Profile is located in.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Indicates whether Traffic View is enabled for the Traffic Manager profile.
        #[builder(into, default)]
        pub traffic_view_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct GetTrafficManagerProfileResult {
        /// This block specifies the DNS configuration of the Profile.
        pub dns_configs: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::network::GetTrafficManagerProfileDnsConfig>,
        >,
        /// The FQDN of the created Profile.
        pub fqdn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// This block specifies the Endpoint monitoring configuration for the Profile.
        pub monitor_configs: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::network::GetTrafficManagerProfileMonitorConfig,
            >,
        >,
        /// The name of the custom header.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The status of the profile.
        pub profile_status: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the algorithm used to route traffic.
        pub traffic_routing_method: pulumi_gestalt_rust::Output<String>,
        /// Indicates whether Traffic View is enabled for the Traffic Manager profile.
        pub traffic_view_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTrafficManagerProfileArgs,
    ) -> GetTrafficManagerProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let traffic_view_enabled_binding = args.traffic_view_enabled.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:network/getTrafficManagerProfile:getTrafficManagerProfile"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficViewEnabled".into(),
                    value: traffic_view_enabled_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTrafficManagerProfileResult {
            dns_configs: o.get_field("dnsConfigs"),
            fqdn: o.get_field("fqdn"),
            id: o.get_field("id"),
            monitor_configs: o.get_field("monitorConfigs"),
            name: o.get_field("name"),
            profile_status: o.get_field("profileStatus"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
            traffic_routing_method: o.get_field("trafficRoutingMethod"),
            traffic_view_enabled: o.get_field("trafficViewEnabled"),
        }
    }
}
