/// Manages a Kubernetes Fleet Update Strategy.
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
pub mod fleet_update_strategy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetUpdateStrategyArgs {
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        #[builder(into)]
        pub kubernetes_fleet_manager_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this Kubernetes Fleet Update Strategy. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// One or more `stage` blocks as defined below.
        #[builder(into)]
        pub stages: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::containerservice::FleetUpdateStrategyStage>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetUpdateStrategyResult {
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        pub kubernetes_fleet_manager_id: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this Kubernetes Fleet Update Strategy. Changing this forces a new Kubernetes Fleet Update Strategy to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `stage` blocks as defined below.
        pub stages: pulumi_wasm_rust::Output<
            Vec<super::super::types::containerservice::FleetUpdateStrategyStage>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FleetUpdateStrategyArgs,
    ) -> FleetUpdateStrategyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let kubernetes_fleet_manager_id_binding = args
            .kubernetes_fleet_manager_id
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let stages_binding = args.stages.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/fleetUpdateStrategy:FleetUpdateStrategy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "kubernetesFleetManagerId".into(),
                    value: &kubernetes_fleet_manager_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "stages".into(),
                    value: &stages_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "kubernetesFleetManagerId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "stages".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FleetUpdateStrategyResult {
            kubernetes_fleet_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesFleetManagerId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            stages: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("stages").unwrap(),
            ),
        }
    }
}
