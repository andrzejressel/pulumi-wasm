/// Manages a Chaos Studio Capability.
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
///       identity:
///         type: example-value
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleTarget:
///     type: azure:chaosstudio:Target
///     name: example
///     properties:
///       location: ${exampleResourceGroup.location}
///       targetResourceId: ${example.id}
///       targetType: example-value
///   exampleCapability:
///     type: azure:chaosstudio:Capability
///     name: example
///     properties:
///       capabilityType: example-value
///       chaosStudioTargetId: ${exampleTarget.id}
/// ```
///
/// ## Import
///
/// An existing Chaos Studio Target can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:chaosstudio/capability:Capability example /{scope}/providers/Microsoft.Chaos/targets/{targetName}/capabilities/{capabilityName}
/// ```
///
/// * Where `{scope}` is the ID of the Azure Resource under which the Chaos Studio Target exists. For example `/subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/some-resource-group`.
///
/// * Where `{targetName}` is the name of the Target. For example `targetValue`.
///
/// * Where `{capabilityName}` is the name of the Capability. For example `capabilityName`.
///
pub mod capability {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CapabilityArgs {
        /// The capability that should be applied to the Chaos Studio Target. For supported values please see this Chaos Studio [Fault Library](https://learn.microsoft.com/azure/chaos-studio/chaos-studio-fault-library). Changing this forces a new Chaos Studio Capability to be created.
        #[builder(into)]
        pub capability_type: pulumi_wasm_rust::Output<String>,
        /// The Chaos Studio Target that the capability should be applied to. Changing this forces a new Chaos Studio Capability to be created.
        #[builder(into)]
        pub chaos_studio_target_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CapabilityResult {
        /// The capability that should be applied to the Chaos Studio Target. For supported values please see this Chaos Studio [Fault Library](https://learn.microsoft.com/azure/chaos-studio/chaos-studio-fault-library). Changing this forces a new Chaos Studio Capability to be created.
        pub capability_type: pulumi_wasm_rust::Output<String>,
        /// The Unique Resource Name of the Capability.
        pub capability_urn: pulumi_wasm_rust::Output<String>,
        /// The Chaos Studio Target that the capability should be applied to. Changing this forces a new Chaos Studio Capability to be created.
        pub chaos_studio_target_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CapabilityArgs) -> CapabilityResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let capability_type_binding = args.capability_type.get_inner();
        let chaos_studio_target_id_binding = args.chaos_studio_target_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:chaosstudio/capability:Capability".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "capabilityType".into(),
                    value: &capability_type_binding,
                },
                register_interface::ObjectField {
                    name: "chaosStudioTargetId".into(),
                    value: &chaos_studio_target_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "capabilityType".into(),
                },
                register_interface::ResultField {
                    name: "capabilityUrn".into(),
                },
                register_interface::ResultField {
                    name: "chaosStudioTargetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CapabilityResult {
            capability_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capabilityType").unwrap(),
            ),
            capability_urn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("capabilityUrn").unwrap(),
            ),
            chaos_studio_target_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chaosStudioTargetId").unwrap(),
            ),
        }
    }
}