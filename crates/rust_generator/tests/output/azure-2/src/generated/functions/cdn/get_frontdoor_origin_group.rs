pub mod get_frontdoor_origin_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorOriginGroupArgs {
        /// Specifies the name of the Front Door Origin Group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Front Door Profile within which Front Door Origin Group exists.
        #[builder(into)]
        pub profile_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrontdoorOriginGroupResult {
        /// Specifies the ID of the Front Door Profile within which this Front Door Origin Group exists.
        pub cdn_frontdoor_profile_id: pulumi_wasm_rust::Output<String>,
        /// A `health_probe` block as defined below.
        pub health_probes: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorOriginGroupHealthProbe>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A `load_balancing` block as defined below.
        pub load_balancings: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::cdn::GetFrontdoorOriginGroupLoadBalancing>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub profile_name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_wasm_rust::Output<
            i32,
        >,
        /// Specifies whether session affinity is enabled on this host.
        pub session_affinity_enabled: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFrontdoorOriginGroupArgs,
    ) -> GetFrontdoorOriginGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let profile_name_binding = args.profile_name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorOriginGroup:getFrontdoorOriginGroup".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "profileName".into(),
                    value: &profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFrontdoorOriginGroupResult {
            cdn_frontdoor_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("cdnFrontdoorProfileId"),
            ),
            health_probes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("healthProbes"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            load_balancings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("loadBalancings"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            profile_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("profileName"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restoreTrafficTimeToHealedOrNewEndpointInMinutes"),
            ),
            session_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sessionAffinityEnabled"),
            ),
        }
    }
}
