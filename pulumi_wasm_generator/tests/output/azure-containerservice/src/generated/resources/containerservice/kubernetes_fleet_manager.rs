/// Manages a Kubernetes Fleet Manager.
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
/// An existing Kubernetes Fleet Manager can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:containerservice/kubernetesFleetManager:KubernetesFleetManager example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ContainerService/fleets/{fleetName}
/// ```
///
/// * Where `{subscriptionId}` is the ID of the Azure Subscription where the Kubernetes Fleet Manager exists. For example `12345678-1234-9876-4563-123456789012`.
///
/// * Where `{resourceGroupName}` is the name of Resource Group where this Kubernetes Fleet Manager exists. For example `example-resource-group`.
///
/// * Where `{fleetName}` is the name of the Fleet. For example `fleetValue`.
///
pub mod kubernetes_fleet_manager {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KubernetesFleetManagerArgs {
        #[builder(into, default)]
        pub hub_profile: pulumi_wasm_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesFleetManagerHubProfile,
            >,
        >,
        /// The Azure Region where the Kubernetes Fleet Manager should exist. Changing this forces a new Kubernetes Fleet Manager to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of this Kubernetes Fleet Manager. Changing this forces a new Kubernetes Fleet Manager to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Resource Group within which this Kubernetes Fleet Manager should exist. Changing this forces a new Kubernetes Fleet Manager to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Kubernetes Fleet Manager.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct KubernetesFleetManagerResult {
        pub hub_profile: pulumi_wasm_rust::Output<
            Option<
                super::super::types::containerservice::KubernetesFleetManagerHubProfile,
            >,
        >,
        /// The Azure Region where the Kubernetes Fleet Manager should exist. Changing this forces a new Kubernetes Fleet Manager to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Kubernetes Fleet Manager. Changing this forces a new Kubernetes Fleet Manager to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Kubernetes Fleet Manager should exist. Changing this forces a new Kubernetes Fleet Manager to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Kubernetes Fleet Manager.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: KubernetesFleetManagerArgs,
    ) -> KubernetesFleetManagerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let hub_profile_binding = args.hub_profile.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:containerservice/kubernetesFleetManager:KubernetesFleetManager"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "hubProfile".into(),
                    value: &hub_profile_binding,
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "hubProfile".into(),
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
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        KubernetesFleetManagerResult {
            hub_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hubProfile").unwrap(),
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
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}