#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod next_generation_firewall_virtual_hub_panorama {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubPanoramaArgs {
        #[builder(into, default)]
        pub destination_nats: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaDestinationNat,
                >,
            >,
        >,
        #[builder(into, default)]
        pub dns_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaDnsSettings,
            >,
        >,
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub network_profile: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaNetworkProfile,
        >,
        #[builder(into)]
        pub panorama_base64_config: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct NextGenerationFirewallVirtualHubPanoramaResult {
        pub destination_nats: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaDestinationNat,
                >,
            >,
        >,
        pub dns_settings: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaDnsSettings,
            >,
        >,
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub network_profile: pulumi_gestalt_rust::Output<
            super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaNetworkProfile,
        >,
        pub panorama_base64_config: pulumi_gestalt_rust::Output<String>,
        pub panoramas: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::paloalto::NextGenerationFirewallVirtualHubPanoramaPanorama,
            >,
        >,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NextGenerationFirewallVirtualHubPanoramaArgs,
    ) -> NextGenerationFirewallVirtualHubPanoramaResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_nats_binding = args.destination_nats.get_output(context);
        let dns_settings_binding = args.dns_settings.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_profile_binding = args.network_profile.get_output(context);
        let panorama_base64_config_binding = args
            .panorama_base64_config
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:paloalto/nextGenerationFirewallVirtualHubPanorama:NextGenerationFirewallVirtualHubPanorama"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationNats".into(),
                    value: destination_nats_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dnsSettings".into(),
                    value: dns_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "networkProfile".into(),
                    value: network_profile_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "panoramaBase64Config".into(),
                    value: panorama_base64_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        NextGenerationFirewallVirtualHubPanoramaResult {
            destination_nats: o.get_field("destinationNats"),
            dns_settings: o.get_field("dnsSettings"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            network_profile: o.get_field("networkProfile"),
            panorama_base64_config: o.get_field("panoramaBase64Config"),
            panoramas: o.get_field("panoramas"),
            resource_group_name: o.get_field("resourceGroupName"),
            tags: o.get_field("tags"),
        }
    }
}
