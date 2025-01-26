pub mod get_host_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetHostPoolArgs {
        /// The name of the Virtual Desktop Host Pool to retrieve.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group where the Virtual Desktop Host Pool exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetHostPoolResult {
        /// The custom RDP properties string for the Virtual Desktop Host Pool.
        pub custom_rdp_properties: pulumi_wasm_rust::Output<String>,
        /// The description for the Virtual Desktop Host Pool.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The friendly name for the Virtual Desktop Host Pool.
        pub friendly_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The type of load balancing performed by the Host Pool
        pub load_balancer_type: pulumi_wasm_rust::Output<String>,
        /// The location/region where the Virtual Desktop Host Pool is located.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The maximum number of users that can have concurrent sessions on a session host.
        pub maximum_sessions_allowed: pulumi_wasm_rust::Output<i32>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The type of personal desktop assignment in use by the Host Pool
        pub personal_desktop_assignment_type: pulumi_wasm_rust::Output<String>,
        /// The preferred Application Group type for the Virtual Desktop Host Pool.
        pub preferred_app_group_type: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `scheduled_agent_updates` block as defined below.
        pub scheduled_agent_updates: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::desktopvirtualization::GetHostPoolScheduledAgentUpdate,
            >,
        >,
        /// Returns `true` if the Start VM on Connection Feature is enabled.
        pub start_vm_on_connect: pulumi_wasm_rust::Output<bool>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The type of the Virtual Desktop Host Pool.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Returns `true` if the Host Pool is in Validation mode.
        pub validate_environment: pulumi_wasm_rust::Output<bool>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetHostPoolArgs,
    ) -> GetHostPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:desktopvirtualization/getHostPool:getHostPool".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "customRdpProperties".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "loadBalancerType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "maximumSessionsAllowed".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "personalDesktopAssignmentType".into(),
                },
                register_interface::ResultField {
                    name: "preferredAppGroupType".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "scheduledAgentUpdates".into(),
                },
                register_interface::ResultField {
                    name: "startVmOnConnect".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
                register_interface::ResultField {
                    name: "validateEnvironment".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetHostPoolResult {
            custom_rdp_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRdpProperties").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            load_balancer_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("loadBalancerType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            maximum_sessions_allowed: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maximumSessionsAllowed").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            personal_desktop_assignment_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("personalDesktopAssignmentType").unwrap(),
            ),
            preferred_app_group_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredAppGroupType").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            scheduled_agent_updates: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scheduledAgentUpdates").unwrap(),
            ),
            start_vm_on_connect: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("startVmOnConnect").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
            validate_environment: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("validateEnvironment").unwrap(),
            ),
        }
    }
}
