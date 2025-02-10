/// Manages a Virtual Desktop Scaling Plan Host Pool Association.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: rg-example-virtualdesktop
///       location: West Europe
///   exampleAssignment:
///     type: azure:authorization:Assignment
///     name: example
///     properties:
///       scope: ${exampleResourceGroup.id}
///       roleDefinitionName: Desktop Virtualization Power On Off Contributor
///       principalId: ${example.objectId}
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
///       friendlyName: Scaling Plan Test
///       description: Test Scaling Plan
///       timeZone: GMT Standard Time
///       schedules:
///         - name: Weekdays
///           daysOfWeeks:
///             - Monday
///             - Tuesday
///             - Wednesday
///             - Thursday
///             - Friday
///           rampUpStartTime: 06:00
///           rampUpLoadBalancingAlgorithm: BreadthFirst
///           rampUpMinimumHostsPercent: 20
///           rampUpCapacityThresholdPercent: 10
///           peakStartTime: 09:00
///           peakLoadBalancingAlgorithm: BreadthFirst
///           rampDownStartTime: 18:00
///           rampDownLoadBalancingAlgorithm: BreadthFirst
///           rampDownMinimumHostsPercent: 10
///           rampDownForceLogoffUsers: false
///           rampDownWaitTimeMinutes: 45
///           rampDownNotificationMessage: Please log of in the next 45 minutes...
///           rampDownCapacityThresholdPercent: 5
///           rampDownStopHostsWhen: ZeroSessions
///           offPeakStartTime: 22:00
///           offPeakLoadBalancingAlgorithm: BreadthFirst
///     options:
///       dependsOn:
///         - ${exampleAssignment}
///   exampleScalingPlanHostPoolAssociation:
///     type: azure:desktopvirtualization:ScalingPlanHostPoolAssociation
///     name: example
///     properties:
///       hostPoolId: ${exampleHostPool.id}
///       scalingPlanId: ${exampleScalingPlan.id}
///       enabled: true
///     options:
///       dependsOn:
///         - ${exampleAssignment}
/// variables:
///   example:
///     fn::invoke:
///       function: azuread:getServicePrincipal
///       arguments:
///         displayName: Windows Virtual Desktop
/// ```
///
/// ## Import
///
/// Associations between Virtual Desktop Scaling Plans and Virtual Desktop Host Pools can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:desktopvirtualization/scalingPlanHostPoolAssociation:ScalingPlanHostPoolAssociation example "/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.DesktopVirtualization/scalingPlans/plan1|/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/myGroup1/providers/Microsoft.DesktopVirtualization/hostPools/myhostpool"
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod scaling_plan_host_pool_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ScalingPlanHostPoolAssociationArgs {
        /// Should the Scaling Plan be enabled on this Host Pool.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The resource ID for the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        #[builder(into)]
        pub host_pool_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource ID for the Virtual Desktop Scaling Plan. Changing this forces a new resource to be created.
        #[builder(into)]
        pub scaling_plan_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ScalingPlanHostPoolAssociationResult {
        /// Should the Scaling Plan be enabled on this Host Pool.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The resource ID for the Virtual Desktop Host Pool. Changing this forces a new resource to be created.
        pub host_pool_id: pulumi_gestalt_rust::Output<String>,
        /// The resource ID for the Virtual Desktop Scaling Plan. Changing this forces a new resource to be created.
        pub scaling_plan_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ScalingPlanHostPoolAssociationArgs,
    ) -> ScalingPlanHostPoolAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let enabled_binding = args.enabled.get_output(context);
        let host_pool_id_binding = args.host_pool_id.get_output(context);
        let scaling_plan_id_binding = args.scaling_plan_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:desktopvirtualization/scalingPlanHostPoolAssociation:ScalingPlanHostPoolAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostPoolId".into(),
                    value: host_pool_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scalingPlanId".into(),
                    value: scaling_plan_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ScalingPlanHostPoolAssociationResult {
            enabled: o.get_field("enabled"),
            host_pool_id: o.get_field("hostPoolId"),
            scaling_plan_id: o.get_field("scalingPlanId"),
        }
    }
}
