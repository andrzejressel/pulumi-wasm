/// Manages a Virtual Desktop Scaling Plan.
///
/// ## Disclaimers
///
/// > **Note** Scaling Plans are currently in preview and are only supported in a limited number of regions. Both the Scaling Plan and any referenced Host Pools must be deployed in a supported region. [Autoscale (preview) for Azure Virtual Desktop host pools](https://docs.microsoft.com/azure/virtual-desktop/autoscale-scaling-plan).
///
/// > **Note** Scaling Plans require specific permissions to be granted to the Windows Virtual Desktop application before a 'host_pool' can be configured. [Required Permissions for Scaling Plans](https://docs.microsoft.com/azure/virtual-desktop/autoscale-scaling-plan#create-a-custom-rbac-role).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRandomUuid:
///     type: random:RandomUuid
///     name: example
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleRoleDefinition:
///     type: azure:authorization:RoleDefinition
///     name: example
///     properties:
///       name: AVD-AutoScale
///       scope: ${exampleResourceGroup.id}
///       description: AVD AutoScale Role
///       permissions:
///         - actions:
///             - Microsoft.Insights/eventtypes/values/read
///             - Microsoft.Compute/virtualMachines/deallocate/action
///             - Microsoft.Compute/virtualMachines/restart/action
///             - Microsoft.Compute/virtualMachines/powerOff/action
///             - Microsoft.Compute/virtualMachines/start/action
///             - Microsoft.Compute/virtualMachines/read
///             - Microsoft.DesktopVirtualization/hostpools/read
///             - Microsoft.DesktopVirtualization/hostpools/write
///             - Microsoft.DesktopVirtualization/hostpools/sessionhosts/read
///             - Microsoft.DesktopVirtualization/hostpools/sessionhosts/write
///             - Microsoft.DesktopVirtualization/hostpools/sessionhosts/usersessions/delete
///             - Microsoft.DesktopVirtualization/hostpools/sessionhosts/usersessions/read
///             - Microsoft.DesktopVirtualization/hostpools/sessionhosts/usersessions/sendMessage/action
///             - Microsoft.DesktopVirtualization/hostpools/sessionhosts/usersessions/read
///           notActions: []
///       assignableScopes:
///         - ${exampleResourceGroup.id}
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       name: ${exampleRandomUuid.result}
///       scope: ${exampleResourceGroup.id}
///       roleDefinitionId: ${exampleRoleDefinition.roleDefinitionResourceId}
///       principalId: ${example.id}
///       skipServicePrincipalAadCheck: true
///   exampleHostPool:
///     type: azure:desktopvirtualization:HostPool
///     name: example
///     properties:
///       name: example-hostpool
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       type: Pooled
///       validateEnvironment: true
///       loadBalancerType: BreadthFirst
///   exampleScalingPlan:
///     type: azure:desktopvirtualization:ScalingPlan
///     name: example
///     properties:
///       name: example-scaling-plan
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       friendlyName: Scaling Plan Example
///       description: Example Scaling Plan
///       timeZone: GMT Standard Time
///       schedules:
///         - name: Weekdays
///           daysOfWeeks:
///             - Monday
///             - Tuesday
///             - Wednesday
///             - Thursday
///             - Friday
///           rampUpStartTime: 05:00
///           rampUpLoadBalancingAlgorithm: BreadthFirst
///           rampUpMinimumHostsPercent: 20
///           rampUpCapacityThresholdPercent: 10
///           peakStartTime: 09:00
///           peakLoadBalancingAlgorithm: BreadthFirst
///           rampDownStartTime: 19:00
///           rampDownLoadBalancingAlgorithm: DepthFirst
///           rampDownMinimumHostsPercent: 10
///           rampDownForceLogoffUsers: false
///           rampDownWaitTimeMinutes: 45
///           rampDownNotificationMessage: Please log off in the next 45 minutes...
///           rampDownCapacityThresholdPercent: 5
///           rampDownStopHostsWhen: ZeroSessions
///           offPeakStartTime: 22:00
///           offPeakLoadBalancingAlgorithm: DepthFirst
///       hostPools:
///         - hostpoolId: ${exampleHostPool.id}
///           scalingPlanEnabled: true
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Azure Virtual Desktop
/// ```
///
/// ## Import
///
/// Virtual Desktop Scaling Plans can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/scalingPlan:ScalingPlan example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.DesktopVirtualization/scalingPlans/plan1
/// ```
///
pub mod scaling_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScalingPlanArgs {
        /// A description of the Scaling Plan.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the tag associated with the VMs you want to exclude from autoscaling.
        #[builder(into, default)]
        pub exclusion_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// Friendly name of the Scaling Plan.
        #[builder(into, default)]
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `host_pool` blocks as defined below.
        #[builder(into, default)]
        pub host_pools: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::desktopvirtualization::ScalingPlanHostPool>>,
        >,
        /// The Azure Region where the Virtual Desktop Scaling Plan should exist. Changing this forces a new Virtual Desktop Scaling Plan to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for this Virtual Desktop Scaling Plan . Changing this forces a new Virtual Desktop Scaling Plan to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Resource Group where the Virtual Desktop Scaling Plan should exist. Changing this forces a new Virtual Desktop Scaling Plan to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `schedule` blocks as defined below.
        #[builder(into)]
        pub schedules: pulumi_wasm_rust::Output<
            Vec<super::super::types::desktopvirtualization::ScalingPlanSchedule>,
        >,
        /// A mapping of tags which should be assigned to the Virtual Desktop Scaling Plan .
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Time Zone which should be used by the Scaling Plan for time based events, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
        #[builder(into)]
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ScalingPlanResult {
        /// A description of the Scaling Plan.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the tag associated with the VMs you want to exclude from autoscaling.
        pub exclusion_tag: pulumi_wasm_rust::Output<Option<String>>,
        /// Friendly name of the Scaling Plan.
        pub friendly_name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `host_pool` blocks as defined below.
        pub host_pools: pulumi_wasm_rust::Output<
            Vec<super::super::types::desktopvirtualization::ScalingPlanHostPool>,
        >,
        /// The Azure Region where the Virtual Desktop Scaling Plan should exist. Changing this forces a new Virtual Desktop Scaling Plan to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Virtual Desktop Scaling Plan . Changing this forces a new Virtual Desktop Scaling Plan to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the Resource Group where the Virtual Desktop Scaling Plan should exist. Changing this forces a new Virtual Desktop Scaling Plan to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// One or more `schedule` blocks as defined below.
        pub schedules: pulumi_wasm_rust::Output<
            Vec<super::super::types::desktopvirtualization::ScalingPlanSchedule>,
        >,
        /// A mapping of tags which should be assigned to the Virtual Desktop Scaling Plan .
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Time Zone which should be used by the Scaling Plan for time based events, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
        pub time_zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ScalingPlanArgs) -> ScalingPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let exclusion_tag_binding = args.exclusion_tag.get_inner();
        let friendly_name_binding = args.friendly_name.get_inner();
        let host_pools_binding = args.host_pools.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let schedules_binding = args.schedules.get_inner();
        let tags_binding = args.tags.get_inner();
        let time_zone_binding = args.time_zone.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/scalingPlan:ScalingPlan".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "exclusionTag".into(),
                    value: &exclusion_tag_binding,
                },
                register_interface::ObjectField {
                    name: "friendlyName".into(),
                    value: &friendly_name_binding,
                },
                register_interface::ObjectField {
                    name: "hostPools".into(),
                    value: &host_pools_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "schedules".into(),
                    value: &schedules_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timeZone".into(),
                    value: &time_zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "exclusionTag".into(),
                },
                register_interface::ResultField {
                    name: "friendlyName".into(),
                },
                register_interface::ResultField {
                    name: "hostPools".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "schedules".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "timeZone".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ScalingPlanResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            exclusion_tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("exclusionTag").unwrap(),
            ),
            friendly_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("friendlyName").unwrap(),
            ),
            host_pools: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostPools").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedules").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            time_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeZone").unwrap(),
            ),
        }
    }
}
