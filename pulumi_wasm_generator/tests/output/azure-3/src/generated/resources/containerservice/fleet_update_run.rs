/// Manages a Kubernetes Fleet Update Run.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: westeurope
///   exampleKubernetesFleetManager:
///     type: azure:containerservice:KubernetesFleetManager
///     name: example
///     properties:
///       location: ${example.location}
///       name: example
///       resourceGroupName: ${example.name}
///   exampleKubernetesCluster:
///     type: azure:containerservice:KubernetesCluster
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       dnsPrefix: example
///       defaultNodePool:
///         name: default
///         nodeCount: 1
///         vmSize: Standard_DS2_v2
///       identity:
///         type: SystemAssigned
///   exampleFleetMember:
///     type: azure:containerservice:FleetMember
///     name: example
///     properties:
///       name: example
///       kubernetesFleetId: ${exampleKubernetesFleetManager.id}
///       kubernetesClusterId: ${exampleKubernetesCluster.id}
///       group: example-group
///   exampleFleetUpdateRun:
///     type: azure:containerservice:FleetUpdateRun
///     name: example
///     properties:
///       name: example
///       kubernetesFleetManagerId: ${exampleKubernetesFleetManager.id}
///       managedClusterUpdate:
///         upgrade:
///           type: Full
///           kubernetesVersion: '1.27'
///         nodeImageSelection:
///           type: Latest
///       stages:
///         - name: example
///           groups:
///             - name: example-group
///           afterStageWaitInSeconds: 21
/// ```
///
/// ## Import
///
/// Kubernetes Fleet Update Runs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/fleetUpdateRun:FleetUpdateRun example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resourceGroup1/providers/Microsoft.ContainerService/fleets/fleet1/updateRuns/updateRun1
/// ```
///
pub mod fleet_update_run {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetUpdateRunArgs {
        /// The ID of the Fleet Update Strategy. Only one of `fleet_update_strategy_id` or `stage` can be specified.
        #[builder(into, default)]
        pub fleet_update_strategy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Run to be created.
        #[builder(into)]
        pub kubernetes_fleet_manager_id: pulumi_wasm_rust::Output<String>,
        /// A `managed_cluster_update` block as defined below.
        #[builder(into)]
        pub managed_cluster_update: pulumi_wasm_rust::Output<
            super::super::types::containerservice::FleetUpdateRunManagedClusterUpdate,
        >,
        /// The name which should be used for this Kubernetes Fleet Update Run. Changing this forces a new Kubernetes Fleet Update Run to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// One or more `stage` blocks as defined below. Only one of `stage` or `fleet_update_strategy_id` can be specified.
        #[builder(into, default)]
        pub stages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::FleetUpdateRunStage>>,
        >,
    }
    #[allow(dead_code)]
    pub struct FleetUpdateRunResult {
        /// The ID of the Fleet Update Strategy. Only one of `fleet_update_strategy_id` or `stage` can be specified.
        pub fleet_update_strategy_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Fleet Manager. Changing this forces a new Kubernetes Fleet Update Run to be created.
        pub kubernetes_fleet_manager_id: pulumi_wasm_rust::Output<String>,
        /// A `managed_cluster_update` block as defined below.
        pub managed_cluster_update: pulumi_wasm_rust::Output<
            super::super::types::containerservice::FleetUpdateRunManagedClusterUpdate,
        >,
        /// The name which should be used for this Kubernetes Fleet Update Run. Changing this forces a new Kubernetes Fleet Update Run to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `stage` blocks as defined below. Only one of `stage` or `fleet_update_strategy_id` can be specified.
        pub stages: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::containerservice::FleetUpdateRunStage>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FleetUpdateRunArgs) -> FleetUpdateRunResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let fleet_update_strategy_id_binding = args.fleet_update_strategy_id.get_inner();
        let kubernetes_fleet_manager_id_binding = args
            .kubernetes_fleet_manager_id
            .get_inner();
        let managed_cluster_update_binding = args.managed_cluster_update.get_inner();
        let name_binding = args.name.get_inner();
        let stages_binding = args.stages.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/fleetUpdateRun:FleetUpdateRun".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "fleetUpdateStrategyId".into(),
                    value: &fleet_update_strategy_id_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesFleetManagerId".into(),
                    value: &kubernetes_fleet_manager_id_binding,
                },
                register_interface::ObjectField {
                    name: "managedClusterUpdate".into(),
                    value: &managed_cluster_update_binding,
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
                    name: "fleetUpdateStrategyId".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesFleetManagerId".into(),
                },
                register_interface::ResultField {
                    name: "managedClusterUpdate".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "stages".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FleetUpdateRunResult {
            fleet_update_strategy_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fleetUpdateStrategyId").unwrap(),
            ),
            kubernetes_fleet_manager_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesFleetManagerId").unwrap(),
            ),
            managed_cluster_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("managedClusterUpdate").unwrap(),
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
