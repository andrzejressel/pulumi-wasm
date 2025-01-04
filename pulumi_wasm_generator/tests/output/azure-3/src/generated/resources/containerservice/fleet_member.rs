/// <!-- Note: This documentation is generated. Any manual changes will be overwritten -->
///
/// Manages a Kubernetes Fleet Member.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:containerservice:KubernetesCluster
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///       dnsPrefix: acctestaksexample
///       defaultNodePool:
///         name: example-value
///         nodeCount: example-value
///         vmSize: example-value
///         upgradeSettings:
///           maxSurge: example-value
///       identity:
///         type: example-value
///   exampleKubernetesFleetManager:
///     type: azure:containerservice:KubernetesFleetManager
///     name: example
///     properties:
///       name: example
///       location: ${exampleResourceGroup.location}
///       resourceGroupName: ${exampleResourceGroup.name}
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleFleetMember:
///     type: azure:containerservice:FleetMember
///     name: example
///     properties:
///       kubernetesClusterId: ${example.id}
///       kubernetesFleetId: ${exampleKubernetesFleetManager.id}
///       name: example
/// ```
///
/// ## Import
///
/// An existing Kubernetes Fleet Member can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/fleetMember:FleetMember example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/fleets/{fleetName}/members/{memberName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Kubernetes Fleet Member exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Kubernetes Fleet Member exists. For example `example-resource-group`.
///
/// * Where `{fleetName}` is the name of the Fleet. For example `fleetValue`.
///
/// * Where `{memberName}` is the name of the Member. For example `memberValue`.
///
pub mod fleet_member {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FleetMemberArgs {
        /// The group this member belongs to for multi-cluster update management.
        #[builder(into, default)]
        pub group: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARM resource ID of the cluster that joins the Fleet. Changing this forces a new Kubernetes Fleet Member to be created.
        #[builder(into)]
        pub kubernetes_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Kubernetes Fleet Id within which this Kubernetes Fleet Member should exist. Changing this forces a new Kubernetes Fleet Member to be created.
        #[builder(into)]
        pub kubernetes_fleet_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Kubernetes Fleet Member. Changing this forces a new Kubernetes Fleet Member to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FleetMemberResult {
        /// The group this member belongs to for multi-cluster update management.
        pub group: pulumi_wasm_rust::Output<Option<String>>,
        /// The ARM resource ID of the cluster that joins the Fleet. Changing this forces a new Kubernetes Fleet Member to be created.
        pub kubernetes_cluster_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the Kubernetes Fleet Id within which this Kubernetes Fleet Member should exist. Changing this forces a new Kubernetes Fleet Member to be created.
        pub kubernetes_fleet_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Kubernetes Fleet Member. Changing this forces a new Kubernetes Fleet Member to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FleetMemberArgs) -> FleetMemberResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let group_binding = args.group.get_inner();
        let kubernetes_cluster_id_binding = args.kubernetes_cluster_id.get_inner();
        let kubernetes_fleet_id_binding = args.kubernetes_fleet_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/fleetMember:FleetMember".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "group".into(),
                    value: &group_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesClusterId".into(),
                    value: &kubernetes_cluster_id_binding,
                },
                register_interface::ObjectField {
                    name: "kubernetesFleetId".into(),
                    value: &kubernetes_fleet_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "group".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesClusterId".into(),
                },
                register_interface::ResultField {
                    name: "kubernetesFleetId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FleetMemberResult {
            group: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("group").unwrap(),
            ),
            kubernetes_cluster_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesClusterId").unwrap(),
            ),
            kubernetes_fleet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kubernetesFleetId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}
