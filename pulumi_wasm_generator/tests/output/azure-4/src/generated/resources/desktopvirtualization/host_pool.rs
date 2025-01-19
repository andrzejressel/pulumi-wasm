/// Manages a Virtual Desktop Host Pool.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleHostPool = host_pool::create(
///         "exampleHostPool",
///         HostPoolArgs::builder()
///             .custom_rdp_properties("audiocapturemode:i:1;audiomode:i:0;")
///             .description("Acceptance Test: A pooled host pool - pooleddepthfirst")
///             .friendly_name("pooleddepthfirst")
///             .load_balancer_type("DepthFirst")
///             .location("${example.location}")
///             .maximum_sessions_allowed(50)
///             .name("pooleddepthfirst")
///             .resource_group_name("${example.name}")
///             .scheduled_agent_updates(
///                 HostPoolScheduledAgentUpdates::builder()
///                     .enabled(true)
///                     .schedules(
///                         vec![
///                             HostPoolScheduledAgentUpdatesSchedule::builder()
///                             .dayOfWeek("Saturday").hourOfDay(2).build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .start_vm_on_connect(true)
///             .type_("Pooled")
///             .validate_environment(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Desktop Host Pools can be imported using the `resource id`, e.g.
///
/// text
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/hostPool:HostPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/hostPools/myhostpool
/// ```
///
pub mod host_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostPoolArgs {
        /// A valid custom RDP properties string for the Virtual Desktop Host Pool, available properties can be [found in this article](https://docs.microsoft.com/windows-server/remote/remote-desktop-services/clients/rdp-files).
        #[builder(into, default)]
        pub custom_rdp_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// A description for the Virtual Desktop Host Pool.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A friendly name for the Virtual Desktop Host Pool.
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// `BreadthFirst` load balancing distributes new user sessions across all available session hosts in the host pool. Possible values are `BreadthFirst`, `DepthFirst` and `Persistent`.
        /// `DepthFirst` load balancing distributes new user sessions to an available session host with the highest number of connections but has not reached its maximum session limit threshold.
        /// `Persistent` should be used if the host pool type is `Personal`
        #[builder(into)]
        pub load_balancer_type: pulumi_wasm_rust::Output<String>,
        /// The location/region where the Virtual Desktop Host Pool is located. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// A valid integer value from 0 to 999999 for the maximum number of users that have concurrent sessions on a session host.
        /// Should only be set if the `type` of your Virtual Desktop Host Pool is `Pooled`.
        #[builder(into, default)]
        pub maximum_sessions_allowed: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// `Automatic` assignment – The service will select an available host and assign it to an user. Possible values are `Automatic` and `Direct`. `Direct` Assignment – Admin selects a specific host to assign to an user. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `personal_desktop_assignment_type` is required if the `type` of your Virtual Desktop Host Pool is `Personal`
        #[builder(into, default)]
        pub personal_desktop_assignment_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to specify the preferred Application Group type for the Virtual Desktop Host Pool. Valid options are `None`, `Desktop` or `RailApplications`. Default is `Desktop`.
        #[builder(into, default)]
        pub preferred_app_group_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed for the Virtual Desktop Host Pool. Possible values are `Enabled`, `Disabled`, `EnabledForClientsOnly` and `EnabledForSessionHostsOnly`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `scheduled_agent_updates` block as defined below. This enables control of when Agent Updates will be applied to Session Hosts.
        #[builder(into, default)]
        pub scheduled_agent_updates: pulumi_wasm_rust::Output<
            Option<
                super::super::types::desktopvirtualization::HostPoolScheduledAgentUpdates,
            >,
        >,
        /// Enables or disables the Start VM on Connection Feature. Defaults to `false`.
        #[builder(into, default)]
        pub start_vm_on_connect: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Desktop Host Pool. Valid options are `Personal` or `Pooled`. Changing the type forces a new resource to be created.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Allows you to test service changes before they are deployed to production. Defaults to `false`.
        #[builder(into, default)]
        pub validate_environment: pulumi_wasm_rust::Output<Option<bool>>,
        /// A VM template for session hosts configuration within hostpool. This is a JSON string.
        #[builder(into, default)]
        pub vm_template: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostPoolResult {
        /// A valid custom RDP properties string for the Virtual Desktop Host Pool, available properties can be [found in this article](https://docs.microsoft.com/windows-server/remote/remote-desktop-services/clients/rdp-files).
        pub custom_rdp_properties: pulumi_wasm_rust::Output<Option<String>>,
        /// A description for the Virtual Desktop Host Pool.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A friendly name for the Virtual Desktop Host Pool.
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// `BreadthFirst` load balancing distributes new user sessions across all available session hosts in the host pool. Possible values are `BreadthFirst`, `DepthFirst` and `Persistent`.
        /// `DepthFirst` load balancing distributes new user sessions to an available session host with the highest number of connections but has not reached its maximum session limit threshold.
        /// `Persistent` should be used if the host pool type is `Personal`
        pub load_balancer_type: pulumi_wasm_rust::Output<String>,
        /// The location/region where the Virtual Desktop Host Pool is located. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// A valid integer value from 0 to 999999 for the maximum number of users that have concurrent sessions on a session host.
        /// Should only be set if the `type` of your Virtual Desktop Host Pool is `Pooled`.
        pub maximum_sessions_allowed: pulumi_wasm_rust::Output<Option<i32>>,
        /// The name of the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// `Automatic` assignment – The service will select an available host and assign it to an user. Possible values are `Automatic` and `Direct`. `Direct` Assignment – Admin selects a specific host to assign to an user. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** `personal_desktop_assignment_type` is required if the `type` of your Virtual Desktop Host Pool is `Personal`
        pub personal_desktop_assignment_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Option to specify the preferred Application Group type for the Virtual Desktop Host Pool. Valid options are `None`, `Desktop` or `RailApplications`. Default is `Desktop`.
        pub preferred_app_group_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Whether public network access is allowed for the Virtual Desktop Host Pool. Possible values are `Enabled`, `Disabled`, `EnabledForClientsOnly` and `EnabledForSessionHostsOnly`. Defaults to `Enabled`.
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A `scheduled_agent_updates` block as defined below. This enables control of when Agent Updates will be applied to Session Hosts.
        pub scheduled_agent_updates: pulumi_wasm_rust::Output<
            Option<
                super::super::types::desktopvirtualization::HostPoolScheduledAgentUpdates,
            >,
        >,
        /// Enables or disables the Start VM on Connection Feature. Defaults to `false`.
        pub start_vm_on_connect: pulumi_wasm_rust::Output<Option<bool>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of the Virtual Desktop Host Pool. Valid options are `Personal` or `Pooled`. Changing the type forces a new resource to be created.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Allows you to test service changes before they are deployed to production. Defaults to `false`.
        pub validate_environment: pulumi_wasm_rust::Output<Option<bool>>,
        /// A VM template for session hosts configuration within hostpool. This is a JSON string.
        pub vm_template: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: HostPoolArgs) -> HostPoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let custom_rdp_properties_binding = args.custom_rdp_properties.get_inner();
        let description_binding = args.description.get_inner();
        let friendly_name_binding = args.friendly_name.get_inner();
        let load_balancer_type_binding = args.load_balancer_type.get_inner();
        let location_binding = args.location.get_inner();
        let maximum_sessions_allowed_binding = args.maximum_sessions_allowed.get_inner();
        let name_binding = args.name.get_inner();
        let personal_desktop_assignment_type_binding = args
            .personal_desktop_assignment_type
            .get_inner();
        let preferred_app_group_type_binding = args.preferred_app_group_type.get_inner();
        let public_network_access_binding = args.public_network_access.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let scheduled_agent_updates_binding = args.scheduled_agent_updates.get_inner();
        let start_vm_on_connect_binding = args.start_vm_on_connect.get_inner();
        let tags_binding = args.tags.get_inner();
        let type__binding = args.type_.get_inner();
        let validate_environment_binding = args.validate_environment.get_inner();
        let vm_template_binding = args.vm_template.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/hostPool:HostPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "customRdpProperties".into(),
                    value: &custom_rdp_properties_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "loadBalancerType".into(),
                    value: &load_balancer_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "maximumSessionsAllowed".into(),
                    value: &maximum_sessions_allowed_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "personalDesktopAssignmentType".into(),
                    value: &personal_desktop_assignment_type_binding,
                },
                register_interface::ObjectField {
                    name: "preferredAppGroupType".into(),
                    value: &preferred_app_group_type_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "scheduledAgentUpdates".into(),
                    value: &scheduled_agent_updates_binding,
                },
                register_interface::ObjectField {
                    name: "startVmOnConnect".into(),
                    value: &start_vm_on_connect_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "validateEnvironment".into(),
                    value: &validate_environment_binding,
                },
                register_interface::ObjectField {
                    name: "vmTemplate".into(),
                    value: &vm_template_binding,
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
                    name: "publicNetworkAccess".into(),
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
                register_interface::ResultField {
                    name: "vmTemplate".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HostPoolResult {
            custom_rdp_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("customRdpProperties").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
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
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccess").unwrap(),
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
            vm_template: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vmTemplate").unwrap(),
            ),
        }
    }
}
