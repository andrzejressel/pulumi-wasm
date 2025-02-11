/// Manages a Kubernetes Fleet Update Strategy.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("westeurope")
///             .name("example-rg")
///             .build_struct(),
///     );
///     let exampleFleetUpdateStrategy = fleet_update_strategy::create(
///         "exampleFleetUpdateStrategy",
///         FleetUpdateStrategyArgs::builder()
///             .kubernetes_fleet_manager_id("${exampleKubernetesFleetManager.id}")
///             .name("example")
///             .stages(
///                 vec![
///                     FleetUpdateStrategyStage::builder().afterStageWaitInSeconds(21)
///                     .groups(vec![FleetUpdateStrategyStageGroup::builder()
///                     .name("example-group-1").build_struct(),]).name("example-stage-1")
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleKubernetesFleetManager = kubernetes_fleet_manager::create(
///         "exampleKubernetesFleetManager",
///         KubernetesFleetManagerArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Kubernetes Fleet Update Strategies can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/fleetUpdateStrategy:FleetUpdateStrategy example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.ContainerService/fleets/fleet1/updateStrategies/updateStrategy1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fleet_update_strategy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetUpdateStrategyArgs {
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        #[builder(into)]
        pub kubernetes_fleet_manager_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this Kubernetes Fleet Update Strategy. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// One or more `stage` blocks as defined below.
        #[builder(into)]
        pub stages: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::super::types::containerservice::FleetUpdateStrategyStage>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetUpdateStrategyResult {
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        pub kubernetes_fleet_manager_id: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this Kubernetes Fleet Update Strategy. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// One or more `stage` blocks as defined below.
        pub stages: pulumi_gestalt_rust::Output<
            Vec<super::super::types::containerservice::FleetUpdateStrategyStage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FleetUpdateStrategyArgs,
    ) -> FleetUpdateStrategyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let kubernetes_fleet_manager_id_binding = args
            .kubernetes_fleet_manager_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let stages_binding = args.stages.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:containerservice/fleetUpdateStrategy:FleetUpdateStrategy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kubernetesFleetManagerId".into(),
                    value: &kubernetes_fleet_manager_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stages".into(),
                    value: &stages_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FleetUpdateStrategyResult {
            kubernetes_fleet_manager_id: o.get_field("kubernetesFleetManagerId"),
            name: o.get_field("name"),
            stages: o.get_field("stages"),
        }
    }
}
