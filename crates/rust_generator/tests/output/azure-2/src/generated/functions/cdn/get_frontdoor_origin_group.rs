#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_frontdoor_origin_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorOriginGroupArgs {
        /// Specifies the name of the Front Door Origin Group.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Front Door Profile within which Front Door Origin Group exists.
        #[builder(into)]
        pub profile_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorOriginGroupResult {
        /// Specifies the ID of the Front Door Profile within which this Front Door Origin Group exists.
        pub cdn_frontdoor_profile_id: pulumi_gestalt_rust::Output<String>,
        /// A `health_probe` block as defined below.
        pub health_probes: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorOriginGroupHealthProbe>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A `load_balancing` block as defined below.
        pub load_balancings: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorOriginGroupLoadBalancing>,
        >,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub profile_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        pub restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_gestalt_rust::Output<
            i32,
        >,
        /// Specifies whether session affinity is enabled on this host.
        pub session_affinity_enabled: pulumi_gestalt_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFrontdoorOriginGroupArgs,
    ) -> GetFrontdoorOriginGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let profile_name_binding = args.profile_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:cdn/getFrontdoorOriginGroup:getFrontdoorOriginGroup".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "profileName".into(),
                    value: profile_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFrontdoorOriginGroupResult {
            cdn_frontdoor_profile_id: o.get_field("cdnFrontdoorProfileId"),
            health_probes: o.get_field("healthProbes"),
            id: o.get_field("id"),
            load_balancings: o.get_field("loadBalancings"),
            name: o.get_field("name"),
            profile_name: o.get_field("profileName"),
            resource_group_name: o.get_field("resourceGroupName"),
            restore_traffic_time_to_healed_or_new_endpoint_in_minutes: o
                .get_field("restoreTrafficTimeToHealedOrNewEndpointInMinutes"),
            session_affinity_enabled: o.get_field("sessionAffinityEnabled"),
        }
    }
}
