pub mod get_frontdoor_origin_group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrontdoorOriginGroupArgs {
        /// Specifies the name of the Front Door Origin Group.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Front Door Profile within which Front Door Origin Group exists.
        #[builder(into)]
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Front Door Profile exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetFrontdoorOriginGroupArgs) -> GetFrontdoorOriginGroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let profile_name_binding = args.profile_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:cdn/getFrontdoorOriginGroup:getFrontdoorOriginGroup".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "cdnFrontdoorProfileId".into(),
                },
                register_interface::ResultField {
                    name: "healthProbes".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancings".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "profileName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "restoreTrafficTimeToHealedOrNewEndpointInMinutes".into(),
                },
                register_interface::ResultField {
                    name: "sessionAffinityEnabled".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFrontdoorOriginGroupResult {
            cdn_frontdoor_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cdnFrontdoorProfileId").unwrap(),
            ),
            health_probes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("healthProbes").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            load_balancings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancings").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            restore_traffic_time_to_healed_or_new_endpoint_in_minutes: pulumi_wasm_rust::__private::into_domain(
                hashmap
                    .remove("restoreTrafficTimeToHealedOrNewEndpointInMinutes")
                    .unwrap(),
            ),
            session_affinity_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sessionAffinityEnabled").unwrap(),
            ),
        }
    }
}
