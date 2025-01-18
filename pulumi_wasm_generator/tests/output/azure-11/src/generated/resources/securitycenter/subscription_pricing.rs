/// Manages the Pricing Tier for Azure Security Center in the current subscription.
///
/// > **NOTE:** Deletion of this resource will reset the pricing tier to `Free`
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = subscription_pricing::create(
///         "example",
///         SubscriptionPricingArgs::builder()
///             .resource_type("VirtualMachines")
///             .tier("Standard")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Using Extensions with Defender CSPM
///
/// ```yaml
/// resources:
///   example1:
///     type: azure:securitycenter:SubscriptionPricing
///     properties:
///       tier: Standard
///       resourceType: CloudPosture
///       extensions:
///         - name: ContainerRegistriesVulnerabilityAssessments
///         - name: AgentlessVmScanning
///           additionalExtensionProperties:
///             ExclusionTags: '[]'
///         - name: AgentlessDiscoveryForKubernetes
///         - name: SensitiveDataDiscovery
/// ```
///
/// ## Import
///
/// The pricing tier can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:securitycenter/subscriptionPricing:SubscriptionPricing example /subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Security/pricings/<resource_type>
/// ```
///
pub mod subscription_pricing {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubscriptionPricingArgs {
        /// One or more `extension` blocks as defined below.
        #[builder(into, default)]
        pub extensions: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::securitycenter::SubscriptionPricingExtension>,
            >,
        >,
        /// The resource type this setting affects. Possible values are `Api`, `AppServices`, `ContainerRegistry`, `KeyVaults`, `KubernetesService`, `SqlServers`, `SqlServerVirtualMachines`, `StorageAccounts`, `VirtualMachines`, `Arm`, `Dns`, `OpenSourceRelationalDatabases`, `Containers`, `CosmosDbs` and `CloudPosture`. Defaults to `VirtualMachines`
        #[builder(into, default)]
        pub resource_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource type pricing subplan. Contact your MSFT representative for possible values.
        #[builder(into, default)]
        pub subplan: pulumi_wasm_rust::Output<Option<String>>,
        /// The pricing tier to use. Possible values are `Free` and `Standard`.
        #[builder(into)]
        pub tier: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubscriptionPricingResult {
        /// One or more `extension` blocks as defined below.
        pub extensions: pulumi_wasm_rust::Output<
            Option<
                Vec<super::super::types::securitycenter::SubscriptionPricingExtension>,
            >,
        >,
        /// The resource type this setting affects. Possible values are `Api`, `AppServices`, `ContainerRegistry`, `KeyVaults`, `KubernetesService`, `SqlServers`, `SqlServerVirtualMachines`, `StorageAccounts`, `VirtualMachines`, `Arm`, `Dns`, `OpenSourceRelationalDatabases`, `Containers`, `CosmosDbs` and `CloudPosture`. Defaults to `VirtualMachines`
        pub resource_type: pulumi_wasm_rust::Output<Option<String>>,
        /// Resource type pricing subplan. Contact your MSFT representative for possible values.
        pub subplan: pulumi_wasm_rust::Output<Option<String>>,
        /// The pricing tier to use. Possible values are `Free` and `Standard`.
        pub tier: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SubscriptionPricingArgs,
    ) -> SubscriptionPricingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let extensions_binding = args.extensions.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let subplan_binding = args.subplan.get_inner();
        let tier_binding = args.tier.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:securitycenter/subscriptionPricing:SubscriptionPricing".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "extensions".into(),
                    value: &extensions_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "subplan".into(),
                    value: &subplan_binding,
                },
                register_interface::ObjectField {
                    name: "tier".into(),
                    value: &tier_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "extensions".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "subplan".into(),
                },
                register_interface::ResultField {
                    name: "tier".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubscriptionPricingResult {
            extensions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensions").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            subplan: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subplan").unwrap(),
            ),
            tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tier").unwrap(),
            ),
        }
    }
}
